use std::fs;
use crate::devices::device::Device;

pub struct ScienceFair240 {}

impl Device for ScienceFair240 {
    fn product_id(&self) -> String {
        "ScienceFair/0.1".to_string()
    }

    fn serial_number(&self) -> String {
        fs::read_to_string("/boot/corevox/serial_number").unwrap_or("UNREGISTERED".to_string())
    }

    fn max_framerate(&self) -> i32 {
        48
    }

    fn pov_frequency(&self) -> i32 {
        24
    }

    fn vox_size(&self) -> [i32; 3] {
        [240, 240, 2]
    }
}
