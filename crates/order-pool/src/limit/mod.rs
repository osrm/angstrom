use std::fmt::Debug;

use guard_types::{
    orders::{
        OrderId, OrderLocation, OrderPriorityData, PoolOrder, PooledComposableOrder,
        PooledLimitOrder, ValidatedOrder
    },
    primitive::PoolId
};
use reth_primitives::B256;

use self::{composable::ComposableLimitPool, limit::LimitPool};
use crate::{
    common::{SizeTracker, ValidOrder},
    BidsAndAsks
};

mod composable;
mod limit;
mod parked;
mod pending;

pub type RegularAndLimit<T, C> = (Vec<T>, Vec<C>);

#[allow(dead_code)]
pub struct LimitOrderPool<O, C>
where
    O: PooledLimitOrder,
    C: PooledComposableOrder + PooledLimitOrder
{
    /// Sub-pool of all limit orders
    limit_orders:      LimitPool<O>,
    /// Sub-pool of all composable orders
    composable_orders: ComposableLimitPool<C>,
    /// The size of the current transactions.
    size:              SizeTracker
}

impl<O: PooledLimitOrder, C: PooledComposableOrder + PooledLimitOrder> LimitOrderPool<O, C>
where
    O: PooledLimitOrder<ValidationData = OrderPriorityData>,
    C: PooledComposableOrder + PooledLimitOrder<ValidationData = OrderPriorityData>
{
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            composable_orders: ComposableLimitPool::new(),
            limit_orders:      LimitPool::new(),
            size:              SizeTracker { max: max_size, current: 0 }
        }
    }

    #[allow(dead_code)]
    pub fn add_composable_order(&mut self, order: ValidOrder<C>) -> Result<(), LimitPoolError<C>> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(LimitPoolError::MaxSize(order.order))
        }

        self.composable_orders.add_order(order)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn add_limit_order(&mut self, order: ValidOrder<O>) -> Result<(), LimitPoolError<O>> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(LimitPoolError::MaxSize(order.order))
        }

        let _location = self.limit_orders.add_order(order)?;

        Ok(())

        // TODO: What do we want to return, how do we want to wire it up
        // so it bubbles up to the highest level
    }

    // individual fetches
    #[allow(dead_code)]
    pub fn fetch_all_pool_orders(
        &mut self,
        id: &PoolId
    ) -> RegularAndLimit<ValidOrder<O>, ValidOrder<C>> {
        (
            self.limit_orders.fetch_all_pool_orders(id),
            self.composable_orders.fetch_all_pool_orders(id)
        )
    }

    pub fn fetch_all_vanilla_orders(&self) -> Vec<BidsAndAsks<O>> {
        self.limit_orders.fetch_bids_asks_per_pool()
    }

    pub fn fetch_all_composable_orders(&self) -> Vec<BidsAndAsks<C>> {
        self.composable_orders.fetch_bids_asks_per_pool()
    }

    #[allow(dead_code)]
    pub fn remove_limit_order(&mut self, order_id: &OrderId) -> Option<ValidOrder<O>> {
        todo!()
    }

    pub fn remove_composable_limit_order(&mut self, order_id: &OrderId) -> Option<ValidOrder<C>> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum LimitPoolError<O: Debug> {
    #[error(
        "Pool has reached max size, and order doesn't satisify replacment requirements, Order: \
         {0:#?}"
    )]
    MaxSize(O),
    #[error("No pool was found for address: {0} Order: {1:#?}")]
    NoPool(PoolId, O),
    #[error("Already have a ordered with nonce: {0:?}, Order: {1:#?}")]
    DuplicateNonce(OrderId, O),
    #[error("Duplicate order: {0:#?}")]
    DuplicateOrder(O)
}
