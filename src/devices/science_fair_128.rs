use std::fs;
use crate::devices::device::Device;

const MAX_ALLOWED_FRAMERATE: i32 = 180;

pub struct ScienceFair128 {}

impl Device for ScienceFair128 {
    fn product_id(&self) -> String {
        return "ScienceFair/0.1".to_string();
    }

    fn serial_number(&self) -> String {
        return fs::read_to_string("/boot/corevox/serial_number").unwrap_or("UNREGISTERED".to_string());
    }

    fn max_framerate(&self) -> i32 {
        return MAX_ALLOWED_FRAMERATE;
    }

    fn pov_frequency(&self) -> i32 {
        return 24;
    }

    fn frame_size(&self) -> [i32; 2] {
        return [128, 128];
    }
}
