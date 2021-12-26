use std::fs;
use crate::devices::device::Device;

pub struct ScienceFair320 {}

impl Device for ScienceFair320 {
    fn product_id(&self) -> String {
        "ScienceFair/0.2".to_string()
    }

    fn serial_number(&self) -> String {
        fs::read_to_string("/boot/corevox/serial_number").unwrap_or("UNREGISTERED".to_string())
    }

    fn max_framerate(&self) -> i32 {
        312
    }

    fn pov_frequency(&self) -> i32 {
        24
    }

    fn vox_size(&self) -> [i32; 3] {
        [320, 240, 12]
    }
}
