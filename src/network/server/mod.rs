pub mod renderer;

use futures::prelude::*;
use tokio::net::TcpListener;
use tokio_serde::formats::SymmetricalMessagePack;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use crate::devices::device::Device;
use crate::network::server::renderer::Renderer;
use crate::network::messages::{DeviceInformation, VoxPack};

pub struct VoxServer {
    pub address: String,
    pub device: Box<dyn Device>,
    pub renderer: Box<dyn Renderer>,
}

unsafe impl Sync for VoxServer {}

unsafe impl Send for VoxServer {}

impl VoxServer {
    pub fn new(address: String, device: Box<dyn Device>, renderer: Box<dyn Renderer>) -> Self {
        return VoxServer {
            address,
            device,
            renderer,
        };
    }

    pub async fn start_listener(&self) {
        let listener = TcpListener::bind(self.address.as_str()).await.unwrap();

        println!("listening on {:?}", listener.local_addr());

        loop {
            let (socket, _) = listener.accept().await.unwrap();

            let (read, write) = socket.into_split();

            let ld = FramedWrite::new(write, LengthDelimitedCodec::new());
            let mut serialized = tokio_serde::SymmetricallyFramed::new(
                ld,
                SymmetricalMessagePack::<DeviceInformation>::default(),
            );

            serialized.send(self.device.device_information()).await.unwrap();

            // Delimit frames using a length header
            let length_delimited = FramedRead::new(read, LengthDelimitedCodec::new());

            // Deserialize frames
            let mut deserialized = tokio_serde::SymmetricallyFramed::new(
                length_delimited,
                SymmetricalMessagePack::<VoxPack>::default(),
            );

            // Spawn a task that prints all received messages to STDOUT
            while let Some(msg) = deserialized.try_next().await.unwrap() {
                self.renderer.handle_vox_pack(msg);
            }
        }
    }
}
