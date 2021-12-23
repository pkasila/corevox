pub mod devices;
pub mod network;

#[cfg(test)]
mod tests {
    use crate::devices::device::Device;
    use crate::devices::science_fair_128::ScienceFair128;

    #[test]
    fn sf128_device_test() {
        let device = ScienceFair128 {};

        let info = device.device_information();

        assert_eq!(info.product_id, "ScienceFair/0.1".to_string());
        assert_eq!(info.serial_number, "UNREGISTERED".to_string());
        assert_eq!(info.pov_frequency, 24);
        assert_eq!(info.max_framerate, 180);
        assert_eq!(info.frame_size, [128, 128]);
    }
}
