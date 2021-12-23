use crate::devices::device::Device;

pub struct PrototypeDevice {
    pub frame: Frame,
    pub max_framerate: i32,
    pub pov_frequency: i32,
}

pub struct Frame {
    pub x: i32,
    pub y: i32,
}

impl Device for PrototypeDevice {
    fn max_framerate(&self) -> i32 {
        self.max_framerate
    }

    fn pov_frequency(&self) -> i32 {
        self.pov_frequency
    }

    fn frame_size(&self) -> [i32; 2] {
        [self.frame.x, self.frame.y]
    }
}
