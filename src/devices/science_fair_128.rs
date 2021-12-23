use std::fs;
use crate::devices::device::Device;

pub struct ScienceFair128 {}

impl Device for ScienceFair128 {
    fn product_id(&self) -> String {
        "ScienceFair/0.1".to_string()
    }

    fn serial_number(&self) -> String {
        fs::read_to_string("/boot/corevox/serial_number").unwrap_or("UNREGISTERED".to_string())
    }

    fn max_framerate(&self) -> i32 {
        180
    }

    fn pov_frequency(&self) -> i32 {
        24
    }

    fn frame_size(&self) -> [i32; 2] {
        [128, 128]
    }
}
