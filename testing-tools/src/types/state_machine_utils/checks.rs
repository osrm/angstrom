use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use crate::{
    controllers::devnet::{AngstromDevnet, DevnetStateMachine},
    types::StateMachineCheckHookFn
};

pub trait WithCheck<C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    type FunctionOutput = StateMachineCheckHookFn<C>;

    fn check_block(&mut self, block_number: u64);
}

impl<'a, C> WithCheck<C> for DevnetStateMachine<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    fn check_block(&mut self, block_number: u64) {
        let f = move |testnet: &mut AngstromDevnet<C>| testnet.check_block_numbers(block_number);
        self.add_check("check block", f);
    }
}
