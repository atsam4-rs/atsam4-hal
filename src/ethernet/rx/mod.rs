mod descriptor;
use descriptor::RxDescriptor;

mod descriptor_block;
pub use descriptor_block::RxDescriptorBlock;

use super::{
    MTU,
    Receiver,
    VolatileReadWrite,
};

pub enum RxError {
    WouldBlock,
}
