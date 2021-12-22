use std::fs;
use crate::devices::device::Device;
use crate::devices::MINIMAL_Z_RATE;

const MAX_ALLOWED_FRAMERATE: i32 = 180;

pub struct ScienceFair128 {
    pub vox_rate: i32,
}

impl Device for ScienceFair128 {
    fn product_id(&self) -> String {
        return "ScienceFair/0.1".to_string();
    }

    fn serial_number(&self) -> String {
        return fs::read_to_string("/boot/corevox/serial_number").unwrap_or("UNREGISTERED".to_string());
    }

    fn vox_size(&self, z: Option<i32>) -> [i32; 3] {
        return [
            128,
            128,
            z.unwrap_or(7)
        ];
    }

    fn frame_rate(&self, vox_rate: Option<i32>, z: Option<i32>) -> Result<i32, &str> {
        let r = vox_rate.unwrap_or(self.vox_rate) * z.unwrap_or(7) * MINIMAL_Z_RATE;
        if r > MAX_ALLOWED_FRAMERATE {
            return Err("Higher than allowed frame rate");
        }
        return Ok(r);
    }
}
