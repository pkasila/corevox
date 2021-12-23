pub mod renderer;

use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use crate::devices::device::Device;
use crate::network::server::renderer::Renderer;
use crate::network::messages::VoxPack;

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

    pub fn start_listener(&self) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(self.address.as_str())?;

        for stream in listener.incoming() {
            let mut stream = stream?;

            stream.write(&*rmp_serde::to_vec(&self.device.device_information())?)?;
            stream.flush()?;

            loop {
                let mut s: String = "".to_string();
                stream.read_to_string(&mut s)?;
                let pack: VoxPack = rmp_serde::from_read_ref(s.as_bytes())?;

                if pack.z * self.device.pov_frequency() <= self.device.max_framerate() {
                    self.renderer.handle_vox_pack(pack);
                } else {
                    break;
                }
            }
        }

        return Ok(());
    }
}
