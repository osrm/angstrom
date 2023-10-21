use std::{
    collections::VecDeque,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_core::types::{Block, H256};
use futures::{Stream, StreamExt};
use guard_types::{
    consensus::{LeaderProposal, PrePreposeBundle, ProposalCommit},
    on_chain::{BestSolvedBundleData, Evidence, EvidenceError, SubmissionBundle}
};
use thiserror::Error;
use tracing::error;

use crate::{
    evidence::EvidenceCollector,
    round::{propose::ProposeState, RoundState},
    round_robin_algo::RoundRobinAlgo,
    signer::Signer
};

#[derive(Debug)]
pub enum ConsensusMessage {
    /// All guards lock there lower-bound and broadcast it
    PrePropose(PrePreposeBundle),
    /// the leader for this round will send out the vanilla bundle and
    /// lower-bound commit for the round
    Proposal(LeaderProposal),
    /// the commit or nil vote the the lower-bound + vanilla proposal
    Commit(ProposalCommit),
    /// if leader. then the finalized bundle that is sent to builders
    RelaySubmission(SubmissionBundle)
}

#[derive(Debug, Error)]
pub enum ConsensusError {
    #[error("Evidence Module had an Error: {0:#?}")]
    EvidenceError(#[from] EvidenceError)
}

/// The ConsensusCore module handles everything related to consensus.
/// This includes tracking slashable events, other guards commits and votes
/// and submitting to consensus.
/// # Design Principles
/// The main interfacing idea for the ConsensusCore is that this module
/// only operates on truths. What this means is this module avoids doing
/// any comparison, building or evaluation in order to keep it as simple as
/// possible (Of course we cannot rid all of this, however there is always a
/// focus to minimize this). all values that are handed to this module are true.
/// for example, this means that the consensus module doesn't know of any other
/// bundles that this guard has built except for the most profitable one. Nor
/// does it know what the proper pricing for a given storage slot is. We
/// abstract all of this out in order to keep this module as clean as possible
/// as proper functionality is critical here to ensure that Angstrom works
/// properly.
pub struct ConsensusCore {
    /// keeps track of the current round state
    round_state:        RoundState,
    /// leader selection algo
    leader_selection:   RoundRobinAlgo,
    /// collects + formulates evidence of byzantine guards
    evidence_collector: EvidenceCollector,
    /// deals with all signing and signature verification
    signer:             Signer,
    /// messages to share with others
    outbound:           VecDeque<ConsensusMessage>
}

impl ConsensusCore {
    /// returns self but also returns the block that the round robin algo
    /// has historic state up until
    pub async fn new() -> (Self, u64) {
        todo!()
    }

    pub fn new_block(&mut self, block: Arc<Block<H256>>) {
        // need to make sure that this is sequential
        if self.round_state.current_height() + 1 == block.number.unwrap().as_u64() {
            // TODO: wire in guard selection stuff
            let new_leader = self.leader_selection.on_new_block(block.clone());
            let is_leader = self.signer.is_us(&new_leader);

            self.round_state
                .new_height(block.number.unwrap().as_u64(), new_leader, is_leader);
        } else {
            panic!("have a gap in blocks which will break the round robin algo");
        }
    }

    pub fn new_pre_propose(&mut self, commit: PreProposeBundle) {
        todo!()
    }

    pub fn proposal(&mut self, proposal: LeaderProposal) {
        todo!()
    }

    pub fn proposal_commit(&mut self, commit: ProposalCommit) {
        todo!()
    }

    pub fn better_bundle(&mut self, bundle_data: BestSolvedBundleData) {
        todo!()
    }
}

impl Stream for ConsensusCore {
    type Item = Result<ConsensusMessage, ConsensusError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let _ = self.round_state.poll_next_unpin(cx);
        todo!()
    }
}
