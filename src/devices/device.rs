use crate::network::messages::DeviceInformation;

pub trait Device {
    fn product_id(&self) -> String {
        return "PROTOTYPE_DEVICE".to_string();
    }

    fn serial_number(&self) -> String {
        return "00000000-1111-2222-3333-444444555555".to_string();
    }

    fn max_framerate(&self) -> i32;

    fn pov_frequency(&self) -> i32;

    fn vox_size(&self) -> [i32; 3];

    fn device_information(&self) -> DeviceInformation {
        return DeviceInformation {
            product_id: self.product_id(),
            serial_number: self.serial_number(),
            max_framerate: self.max_framerate(),
            pov_frequency: self.pov_frequency(),
            vox_size: self.vox_size(),
        };
    }
}
