use crate::*;

use lru_cache::LruCache;

pub struct ProposalCollector {
    pub proposals: LruCache<u64, ProposalRoundCollector>,
}

impl Default for collection::proposal::ProposalCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalCollector {
    pub fn new() -> Self {
        ProposalCollector {
            proposals: LruCache::new(20),
        }
    }

    pub fn add(&mut self, proposal: Proposal) -> bool {
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

    pub fn get_proposal(&mut self, height: u64, round: u64) -> Option<Proposal> {
        self.proposals
            .get_mut(&height)
            .and_then(|prop| prop.get_proposal(round))
    }
}

pub struct ProposalRoundCollector {
    pub round_proposals: LruCache<u64, Proposal>,
}

impl Default for collection::proposal::ProposalRoundCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalRoundCollector {
    pub fn new() -> Self {
        ProposalRoundCollector {
            round_proposals: LruCache::new(20),
        }
    }

    pub fn add(&mut self, round: u64, proposal: Proposal) -> bool {
        if self.round_proposals.contains_key(&round) {
            false
        } else {
            self.round_proposals.insert(round, proposal);
            true
        }
    }

    pub fn get_proposal(&mut self, round: u64) -> Option<Proposal> {
        self.round_proposals.get_mut(&round).cloned()
    }
}
