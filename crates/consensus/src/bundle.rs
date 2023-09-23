use std::collections::{
    hash_map::{Entry, OccupiedEntry},
    HashMap, HashSet
};

use guard_types::{
    consensus::{Bundle23Votes, BundleVote, GuardSet, Valid23Bundle},
    on_chain::SimmedBundle
};
use reth_primitives::H256;
use tracing::{debug, error, warn};

pub enum BundleVoteMessage {
    SignAndPropagate(H256),
    NewBundle23Votes(Valid23Bundle)
}

/// The bundle vote manager is in-charge for tracking all bundle votes
/// in order to make sure that we are able to reach consensus on the best
/// bundle
pub struct BundleVoteManager {
    best_bundle:        Option<Valid23Bundle>,
    known_bundles:      HashMap<H256, SimmedBundle>,
    known_bundle_votes: HashMap<H256, Vec<BundleVote>>,
    known_23_bundles:   HashSet<H256>,
    guards:             GuardSet
}

impl Default for BundleVoteManager {
    fn default() -> Self {
        Self { ..Default::default() }
    }
}

impl BundleVoteManager {
    pub fn new_simmed_bundle(&mut self, bundle: SimmedBundle) -> Option<BundleVoteMessage> {
        let hash = bundle.raw.clone().into();
        if self.known_23_bundles.contains(&hash) {
            return None
        }

        if self.known_bundles.insert(hash, bundle).is_none() {
            return Some(BundleVoteMessage::SignAndPropagate(hash))
        }

        None
    }

    pub fn new_bundle23(&mut self, bundle: Valid23Bundle) {
        if !bundle.votes.verify_signatures(&self.guards) {
            warn!(?bundle, "bundle was invalid 2/3");
            return
        }
        let hash = bundle.votes.hash;
        self.known_23_bundles.insert(hash);

        // TODO: need to handle case where we don't have bundle yet
        let underlying_bundle = self.known_bundles.remove(&hash).unwrap();

        if let Some(best_bundle) = self.best_bundle.take() {
            if underlying_bundle.get_cumulative_lp_bribe()
                > best_bundle.bundle.get_cumulative_lp_bribe()
            {
                self.best_bundle = Some(bundle);
            }
        } else {
            self.best_bundle = Some(bundle);
        }
    }

    pub fn new_bundle_vote(&mut self, vote: BundleVote) -> Option<BundleVoteMessage> {
        let hash = vote.hash;
        if let Some(new_23) = match self.known_bundle_votes.entry(hash) {
            Entry::Vacant(v) => {
                if !Self::verify_vote(&self.guards, vote) {
                    return None
                }

                let mut entry = Vec::with_capacity(self.guards.len());
                entry.push(vote);
                v.insert(entry);
                None
            }
            Entry::Occupied(mut o) => {
                if o.get()
                    .iter()
                    .find(|f| f.signature == vote.signature)
                    .is_some()
                {
                    debug!("got dup vote");
                    return None
                }
                if !Self::verify_vote(&self.guards, &vote) {
                    return None
                }
                o.get_mut().push(vote);

                return Self::check_for_23(o, &self.guards)
            }
        };
        
        None
    }

    pub fn has_signed_bundle(&self, bundle_hash: &H256) -> bool {
        self.known_bundle_votes.contains_key(bundle_hash)
    }

    fn verify_vote(guards: &GuardSet, vote: &BundleVote) -> bool {
        let Ok(id) = vote
            .recover_public_key()
            // .inspect_err(|e| error!(?e, "failed to recover vote"))
        else {
            return false
        };

        if !guards.contains_key(id) {
            warn!(?vote, "no guard found for recovered signature");
            return false
        }

        true
    }

    fn check_for_23(
        mut entry: OccupiedEntry<'_, H256, Vec<BundleVote>>,
        guards: &GuardSet
    ) -> Option<Bundle23Votes> {
        let total_guards = guards.len();
        // check to see if we have less than 2/3rd
        if entry.get().len() % total_guards <= 66 {
            return None
        }

        let votes = entry.remove();
        let (hash, height, round) = votes
            .first()
            .map(|vote| (vote.hash, vote.height, vote.round))?;

        let signatures = votes
            .into_iter()
            .map(|vote| vote.signature)
            .collect::<Vec<_>>();

        let new_bundle_votes = Bundle23Votes::new(hash, height, round, signatures);

        Some(new_bundle_votes)
    }
}
