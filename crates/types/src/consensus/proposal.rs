use serde::{Deserialize, Serialize};

use crate::{
    consensus::order_buffer::OrderBuffer,
    on_chain::{LowerBound, Signature, VanillaBundle}
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub ethereum_block:   u64,
    pub vanilla_bundle:   VanillaBundle,
    pub lower_bound:      LowerBound,
    pub order_buffer:     Vec<OrderBuffer>,
    /// This signature is over (etheruem_block | hash(vanilla_bundle) |
    /// hash(lower_bound))
    pub leader_signature: Signature
}
