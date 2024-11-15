use alloy_primitives::{Address, Bytes};

use crate::contract_bindings::angstrom::Angstrom::PoolKey;

#[derive(Debug, Clone)]
pub struct InitialTestnetState {
    pub angstrom_addr: Address,
    pub state:         Bytes,
    pub pool_keys:     Vec<PoolKey>
}

impl InitialTestnetState {
    pub fn new(angstrom_addr: Address, state: Bytes, pool_keys: Vec<PoolKey>) -> Self {
        Self { angstrom_addr, state, pool_keys }
    }
}
