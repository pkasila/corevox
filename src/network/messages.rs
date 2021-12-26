use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInformation {
    pub product_id: String,
    pub serial_number: String,
    pub max_framerate: i32,
    pub pov_frequency: i32,
    pub vox_size: [i32; 3],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoxPack {
    pub raw: Vec<u8>,
}
