use crate::*;

use lru_cache::LruCache;

pub struct ProposalCollector<T> {
    pub proposals: LruCache<usize, ProposalRoundCollector<T>>,
}

impl<T> Default for collection::proposal::ProposalCollector<T>
where
    T: Clone + Eq + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ProposalCollector<T>
where
    T: Clone + Eq + PartialEq,
{
    pub fn new() -> Self {
        ProposalCollector {
            proposals: LruCache::new(20),
        }
    }

    pub fn add(&mut self, proposal: Proposal<T>) -> bool {
        let height = proposal.clone().height;
        let round = proposal.clone().round;

        if self.proposals.contains_key(&height) {
            self.proposals
                .get_mut(&height)
                .unwrap()
                .add(round, proposal)
        } else {
            let mut round_proposals = ProposalRoundCollector::new();
            round_proposals.add(round, proposal);
            self.proposals.insert(height, round_proposals);
            true
        }
    }

    pub fn get_proposal(&mut self, height: usize, round: usize) -> Option<Proposal<T>> {
        self.proposals
            .get_mut(&height)
            .and_then(|prop| prop.get_proposal(round))
    }
}

pub struct ProposalRoundCollector<T> {
    pub round_proposals: LruCache<usize, Proposal<T>>,
}

impl<T> Default for collection::proposal::ProposalRoundCollector<T>
where
    T: Clone + Eq + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ProposalRoundCollector<T>
where
    T: Clone + Eq + PartialEq,
{
    pub fn new() -> Self {
        ProposalRoundCollector {
            round_proposals: LruCache::new(20),
        }
    }

    pub fn add(&mut self, round: usize, proposal: Proposal<T>) -> bool {
        if self.round_proposals.contains_key(&round) {
            false
        } else {
            self.round_proposals.insert(round, proposal);
            true
        }
    }

    pub fn get_proposal(&mut self, round: usize) -> Option<Proposal<T>> {
        self.round_proposals.get_mut(&round).cloned()
    }
}
