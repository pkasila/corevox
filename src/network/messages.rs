use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInformation {
    pub product_id: String,
    pub serial_number: String,
    pub max_framerate: i32,
    pub pov_frequency: i32,
    pub frame_size: [i32; 2],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoxPack {
    pub z: i32,
    pub raw: Vec<u8>,
}
