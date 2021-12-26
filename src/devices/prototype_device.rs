use crate::devices::device::Device;

pub struct PrototypeDevice {
    pub frame: Workspace,
    pub max_framerate: i32,
    pub pov_frequency: i32,
}

pub struct Workspace {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Device for PrototypeDevice {
    fn max_framerate(&self) -> i32 {
        self.max_framerate
    }

    fn pov_frequency(&self) -> i32 {
        self.pov_frequency
    }

    fn vox_size(&self) -> [i32; 3] {
        [self.frame.x, self.frame.y, self.frame.z]
    }
}
