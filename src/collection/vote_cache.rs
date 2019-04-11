use crate::{Address, Hash, Vote, VoteType};
use lru_cache::LruCache;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct VoteCache {
    pub(crate) votes: LruCache<u64, RoundCollector>,
    pub(crate) prevote_count: HashMap<u64, usize>,
}

impl VoteCache {
    pub(crate) fn new() -> Self {
        VoteCache {
            votes: LruCache::new(16),
            prevote_count: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, vote: Vote) -> bool {
        let height = vote.height;
        let round = vote.round;
        let vote_type = vote.vote_type;
        let sender = vote.voter;
        let vote = vote.proposal;

        if vote_type == VoteType::Prevote {
            if self.votes.contains_key(&height) {
                if self
                    .votes
                    .get_mut(&height)
                    .unwrap()
                    .add(round, vote_type, sender, vote)
                {
                    // update prevote count hashmap
                    let counter = self.prevote_count.entry(round).or_insert(0);
                    *counter += 1;
                    true
                } else {
                    // if add prevote fail, do not update prevote hashmap
                    false
                }
            } else {
                let mut round_votes = RoundCollector::new();
                round_votes.add(round, vote_type, sender, vote);
                self.votes.insert(height, round_votes);
                // update prevote count hashmap
                let counter = self.prevote_count.entry(round).or_insert(0);
                *counter += 1;
                true
            }
        } else if self.votes.contains_key(&height) {
            self.votes
                .get_mut(&height)
                .unwrap()
                .add(round, vote_type, sender, vote)
        } else {
            let mut round_votes = RoundCollector::new();
            round_votes.add(round, vote_type, sender, vote);
            self.votes.insert(height, round_votes);
            true
        }
    }

    pub(crate) fn get_voteset(
        &mut self,
        height: u64,
        round: u64,
        vote_type: VoteType,
    ) -> Option<VoteSet> {
        self.votes
            .get_mut(&height)
            .and_then(|rc| rc.get_voteset(round, vote_type))
    }

    pub(crate) fn clear_prevote_count(&mut self) {
        self.prevote_count.clear();
    }
}

// 1. sender's vote message  2. proposal's hash  3. count
#[derive(Clone, Debug)]
pub(crate) struct VoteSet {
    pub(crate) votes_by_sender: HashMap<Address, Hash>,
    pub(crate) votes_by_proposal: HashMap<Hash, usize>,
    pub(crate) count: usize,
}

impl VoteSet {
    pub(crate) fn new() -> Self {
        VoteSet {
            votes_by_sender: HashMap::new(),
            votes_by_proposal: HashMap::new(),
            count: 0,
        }
    }

    pub(crate) fn add(&mut self, sender: Address, vote: Hash) -> bool {
        let mut is_add = false;
        self.votes_by_sender.entry(sender).or_insert_with(|| {
            is_add = true;
            vote.to_owned()
        });
        if is_add {
            self.count += 1;
            *self.votes_by_proposal.entry(vote).or_insert(0) += 1;
        }
        is_add
    }

    pub(crate) fn extract_polc(
        &self,
        height: u64,
        round: u64,
        vote_type: VoteType,
        proposal: &[u8],
    ) -> Vec<Vote> {
        // abstract the votes for the polc proposal into a vec
        let mut polc = Vec::new();
        for (address, vote_proposal) in &self.votes_by_sender {
            let proposal = proposal.to_vec();
            if vote_proposal == &proposal {
                polc.push(Vote {
                    vote_type: vote_type.clone(),
                    height,
                    round,
                    proposal: proposal.clone(),
                    voter: address.clone(),
                });
            }
        }
        polc
    }
}

// round -> step collector
#[derive(Debug)]
pub(crate) struct RoundCollector {
    pub(crate) round_votes: LruCache<u64, StepCollector>,
}

impl RoundCollector {
    pub(crate) fn new() -> Self {
        RoundCollector {
            round_votes: LruCache::new(16),
        }
    }

    pub(crate) fn add(
        &mut self,
        round: u64,
        vote_type: VoteType,
        sender: Address,
        vote: Hash,
    ) -> bool {
        if self.round_votes.contains_key(&round) {
            self.round_votes
                .get_mut(&round)
                .unwrap()
                .add(vote_type, sender, vote)
        } else {
            let mut step_votes = StepCollector::new();
            step_votes.add(vote_type, sender, vote);
            self.round_votes.insert(round, step_votes);
            true
        }
    }

    pub(crate) fn get_voteset(&mut self, round: u64, vote_type: VoteType) -> Option<VoteSet> {
        self.round_votes
            .get_mut(&round)
            .and_then(|sc| sc.get_voteset(vote_type))
    }
}

// step -> voteset
#[derive(Debug, Default)]
pub(crate) struct StepCollector {
    pub(crate) step_votes: HashMap<VoteType, VoteSet>,
}

impl StepCollector {
    pub(crate) fn new() -> Self {
        StepCollector {
            step_votes: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, vote_type: VoteType, sender: Address, vote: Hash) -> bool {
        self.step_votes
            .entry(vote_type)
            .or_insert_with(VoteSet::new)
            .add(sender, vote)
    }

    pub(crate) fn get_voteset(&self, vote_type: VoteType) -> Option<VoteSet> {
        self.step_votes.get(&vote_type).cloned()
    }
}
