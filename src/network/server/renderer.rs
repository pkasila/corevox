use crate::network::messages::VoxPack;

pub trait Renderer {
    fn handle_vox_pack(&self, pack: VoxPack);
}