use crate::*;
use crate::{correctness::bft_test::*, error::BftError};

use rand::{thread_rng, Rng};

use std::{collections::HashMap, time::Instant};

pub struct Actuator<T> {
    function: T,

    height: u64,
    round: u64,
    lock_round: Option<u64>,
    authority_list: Vec<Address>,
    proposal: Vec<u8>,
    byzantine: Vec<u8>,
    stime: Instant,
    htime: Instant,
}

impl<T> Actuator<T>
where
    T: Support,
{
    pub fn new(function: T, height: u64, round: u64, authority_list: Vec<Address>) -> Self {
        Actuator {
            function,
            height,
            round,
            lock_round: None,
            authority_list,
            proposal: Vec::new(),
            byzantine: vec![0, 0, 0],
            stime: Instant::now(),
            htime: Instant::now(),
        }
    }

    pub fn set_authority_list(&mut self, authority_list: Vec<Address>) {
        self.authority_list = authority_list;
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    pub fn stop(&self) {
        self.function.stop()
    }

    pub fn proc_test(&mut self, cases: BftTest) -> BftResult<()> {
        for case in cases.iter() {
            if case == &SHOULD_COMMIT {
                if let Some(commit) = self.function.try_get_commit() {
                    if commit.result != self.proposal {
                        return Err(BftError::CommitIncorrect(self.height));
                    }
                }
            } else if case == &NO_COMMIT_BUT_LOCK {
                if self.function.try_get_commit().is_some() {
                    return Err(BftError::CommitInvalid(self.height));
                }
            } else if case == &NO_COMMIT_NO_LOCK {
                if self.function.try_get_commit().is_some() {
                    return Err(BftError::CommitInvalid(self.height));
                }
            } else {
                let prevote = case[0..3].to_vec();
                let precommit = case[3..6].to_vec();
                let proposer = self.function.cal_proposer(self.height, self.round);

                if proposer == 0 {
                    let feed = self.generate_feed();
                    self.proposal = feed.proposal.clone();
                    self.function.send(FrameSend::Feed(feed));
                } else if proposer < self.authority_list.len() {
                    // TODO cache proposal
                    let proposal =
                        self.generate_proposal(proposer, self.lock_round, Vec::new());
                    self.function.send(FrameSend::Proposal(proposal));
                    self.generate_vote(prevote, precommit);
                } else {
                    panic!("Proposer index beyond authority list!");
                }
            }
        }
        println!("Total test time; {:?}", Instant::now() - self.stime);
        Ok(())
    }

    fn generate_feed(&self) -> Feed {
        let mut proposal = vec![0, 0, 0];
        while proposal == self.byzantine {
            let mut rng = thread_rng();
            for ii in proposal.iter_mut() {
                *ii = rng.gen();
            }
        }

        Feed {
            height: self.height,
            proposal,
        }
    }

    fn generate_proposal(
        &mut self,
        auth_index: usize,
        lock_round: Option<u64>,
        lock_votes: Vec<Vote>,
    ) -> Proposal {
        let mut proposal = vec![0, 0, 0];
        while proposal == self.byzantine {
            let mut rng = thread_rng();
            for ii in proposal.iter_mut() {
                *ii = rng.gen();
            }
        }
        self.proposal = proposal.clone();

        Proposal {
            height: self.height,
            round: self.round,
            content: proposal,
            proposer: self.authority_list[auth_index].clone(),
            lock_round,
            lock_votes,
        }
    }

    fn generate_vote(&self, prevote: Vec<u8>, precommit: Vec<u8>) {
        // TODO: cache vote
        for i in 0..2 {
            if prevote[i] == 1 {
                let vote = Vote {
                    height: self.height,
                    round: self.round,
                    vote_type: VoteType::Prevote,
                    proposal: self.proposal.clone(),
                    voter: self.authority_list[i + 1].clone(),
                };
                self.function.send(FrameSend::Vote(vote));
            }
        }

        for i in 0..2 {
            if precommit[i] == 1 {
                let vote = Vote {
                    height: self.height,
                    round: self.round,
                    vote_type: VoteType::Precommit,
                    proposal: self.proposal.clone(),
                    voter: self.authority_list[i + 1].clone(),
                };
                self.function.send(FrameSend::Vote(vote));
            }
        }
    }
}
