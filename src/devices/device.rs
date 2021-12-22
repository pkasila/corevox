use crate::devices::MINIMAL_Z_RATE;
use crate::network::messages::DeviceInformation;

pub trait Device {
    fn product_id(&self) -> String {
        return "PROTOTYPE_DEVICE".to_string();
    }
    fn serial_number(&self) -> String {
        return "00000000-1111-2222-3333-444444555555".to_string();
    }
    fn vox_size(&self, z: Option<i32>) -> [i32; 3];
    fn frame_rate(&self, vox_rate: Option<i32>, z: Option<i32>) -> Result<i32, &str>;
    fn device_information(&self, vox_rate: Option<i32>, z: Option<i32>) -> DeviceInformation {
        return DeviceInformation {
            product_id: self.product_id(),
            serial_number: self.serial_number(),
            max_slices: self.vox_size(None)[2],
            vox_size: self.vox_size(z),
            frame_rate: self.frame_rate(vox_rate, z).unwrap_or(MINIMAL_Z_RATE),
        };
    }
}
