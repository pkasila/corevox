pub mod supplier;

use std::error::Error;
use futures::prelude::*;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{Framed, FramedRead, FramedWrite, LengthDelimitedCodec};
use std::ptr::null;
use tokio::net::tcp::OwnedWriteHalf;
use crate::network::client::supplier::Supplier;
use crate::network::messages::{DeviceInformation, VoxPack};

#[async_trait::async_trait]
pub trait VoxClient {
    fn register_supplier(&self, s: Box<dyn Supplier>);
    async fn send_pack(&mut self, pack: VoxPack) -> Result<(), Box<dyn Error>>;
}

pub struct VoxClientImpl {
    address: String,
    pub device_info: DeviceInformation,
    sender: tokio_serde::Framed<FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>, VoxPack, VoxPack, MessagePack<VoxPack, VoxPack>>
}

impl VoxClientImpl {
    pub async fn new(address: String) -> Result<VoxClientImpl, Box<dyn Error>> {
        let (read, write) = TcpStream::connect(address.as_str()).await.unwrap().into_split();

        println!("Connected!");

        // Delimit frames using a length header
        let length_delimited = FramedRead::new(read, LengthDelimitedCodec::new());

        // Deserialize frames
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalMessagePack::<DeviceInformation>::default(),
        );

        let msg: DeviceInformation = deserialized.try_next().await.unwrap().unwrap();

        println!("Connected to the device {} ({})", msg.product_id, msg.serial_number);

        let ld = FramedWrite::new(write, LengthDelimitedCodec::new());

        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            ld,
            SymmetricalMessagePack::<VoxPack>::default(),
        );

        return Ok(VoxClientImpl {
            address,
            device_info: msg,
            sender: deserialized,
        });
    }
}

#[async_trait::async_trait]
impl VoxClient for VoxClientImpl {
    fn register_supplier(&self, s: Box<dyn Supplier>) {
        s.assign_client(self);
    }

    async fn send_pack(&mut self, pack: VoxPack) -> Result<(), Box<dyn Error>> {
        self.sender.send(pack).await?;
        return Ok(());
    }
}
