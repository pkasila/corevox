pub mod supplier;

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ptr::null;
use crate::network::client::supplier::Supplier;
use crate::network::messages::VoxPack;

pub trait VoxClient {
    fn register_supplier(&self, s: Box<dyn Supplier>);
    fn send_pack(&mut self, pack: VoxPack) -> Result<(), Box<dyn Error>>;
}

pub struct VoxClientImpl {
    address: String,
    stream: Box<TcpStream>,
}

impl VoxClientImpl {
    pub fn new(address: String) -> Result<VoxClientImpl, Box<dyn Error>> {
        let mut stream = Box::new(TcpStream::connect(address.as_str())?);

        let mut s = "".to_string();
        stream.read_to_string(&mut s)?;

        return Ok(VoxClientImpl {
            address,
            stream,
        });
    }
}

impl VoxClient for VoxClientImpl {
    fn register_supplier(&self, s: Box<dyn Supplier>) {
        s.assign_client(self);
    }

    fn send_pack(&mut self, pack: VoxPack) -> Result<(), Box<dyn Error>> {
        self.stream.write(&*rmp_serde::to_vec(&pack)?)?;
        self.stream.flush()?;
        return Ok(());
    }
}
