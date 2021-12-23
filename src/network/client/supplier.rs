use crate::network::client::VoxClient;

pub trait Supplier {
    fn assign_client(&self, delegate: &dyn VoxClient);
}
