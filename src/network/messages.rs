use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionInitialization {
    pub z: i32,
    pub vox_rate: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceInformation {
    pub product_id: String,
    pub serial_number: String,
    pub max_slices: i32,
    pub vox_size: [i32; 3],
    pub frame_rate: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoxPack {
    pub frame_rate: i32,
    pub raw: Vec<u8>,
}
