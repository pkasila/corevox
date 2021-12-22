use crate::devices::device::Device;
use crate::devices::MINIMAL_Z_RATE;

pub struct PrototypeDevice {
    pub workspace: Workspace,
    pub vox_rate: i32,
    pub z_rate: Option<i32>,
}

pub struct Workspace {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Device for PrototypeDevice {
    fn vox_size(&self, z: Option<i32>) -> [i32; 3] {
        return [
            self.workspace.x,
            self.workspace.y,
            z.unwrap_or(self.workspace.z)
        ];
    }

    fn frame_rate(&self, vox_rate: Option<i32>, z: Option<i32>) -> Result<i32, &str> {
        return Ok(vox_rate.unwrap_or(self.vox_rate) * z.unwrap_or(self.workspace.z) * self.z_rate.unwrap_or(MINIMAL_Z_RATE));
    }
}
