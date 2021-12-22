pub mod devices;
pub mod network;

#[cfg(test)]
mod tests {
    use crate::devices::device::Device;
    use crate::devices::prototype_device::{PrototypeDevice, Workspace};
    use crate::devices::science_fair_128::ScienceFair128;

    #[test]
    fn prototype_device_test() {
        let device = PrototypeDevice {
            workspace: Workspace {
                x: 128,
                y: 128,
                z: 7,
            },
            vox_rate: 1,
            z_rate: None,
        };

        assert_eq!(device.device_information(Option::from(1), Option::from(7)).frame_rate, 168);
    }

    #[test]
    fn sf128_device_test() {
        let device = ScienceFair128 {
            vox_rate: 1
        };

        let info = device.device_information(Option::from(1), Option::from(6));

        assert_eq!(info.product_id, "ScienceFair/0.1".to_string());
        assert_eq!(info.serial_number, "UNREGISTERED".to_string());
        assert_eq!(info.max_slices, 7);
        assert_eq!(info.vox_size, [128, 128, 6]);
        assert_eq!(info.frame_rate, 144);
    }
}
