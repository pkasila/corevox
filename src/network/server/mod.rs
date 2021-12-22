pub mod renderer;

use std::error::Error;
use std::hash::Hasher;
use std::io::{Read, Write};
use std::net::TcpListener;
use crate::devices::device::Device;
use crate::network::server::renderer::Renderer;
use crate::network::messages::VoxPack;

trait VoxServer {
    fn new(address: String, device: Box<dyn Device>, renderer: Box<dyn Renderer>) -> Self;
    fn start_listener(&self) -> Result<(), Box<dyn Error>>;
}

struct VoxServerImpl {
    address: String,
    device: Box<dyn Device>,
    renderer: Box<dyn Renderer>,
}

unsafe impl Sync for VoxServerImpl {

}

unsafe impl Send for VoxServerImpl {

}

impl VoxServer for VoxServerImpl {
    fn new(address: String, device: Box<dyn Device>, renderer: Box<dyn Renderer>) -> Self {
        return VoxServerImpl {
            address,
            device,
            renderer
        };
    }

    fn start_listener(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.address.as_str())?;

        for stream in listener.incoming() {
            let mut stream = stream?;

            stream.write(&*rmp_serde::to_vec(&self.device.device_information(None, None))?)?;

            loop {
                let mut s: String = "".to_string();
                stream.read_to_string(&mut s)?;
                let pack: VoxPack = rmp_serde::from_read_ref(s.as_bytes())?;

                self.renderer.handle_vox_pack(pack);
            }
        }

        return Ok(());
    }
}
