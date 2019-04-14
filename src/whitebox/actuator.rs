use crate::whitebox::{
    collection::{storage::*, util::Msg, vote_cache::VoteCache},
    correctness::test_case::*,
    error::BftError,
    *,
};

use rand::{thread_rng, Rng};
use time::Timespec;

/// A whitebox testing actuator.
pub struct Actuator<T> {
    function: T,

    height: u64,
    round: u64,
    lock_round: Option<u64>,
    lock_proposal: Option<Vec<u8>>,
    authority_list: Vec<Address>,
    proposal: Vec<u8>,
    byzantine: Vec<Vec<u8>>,
    storage: Storage,
    vote_cache: VoteCache,
    stime: Timespec,
    htime: Timespec,
}

impl<T> Actuator<T>
where
    T: Support,
{
    /// A function to create a new testing acutator.
    pub fn new(
        function: T,
        height: u64,
        round: u64,
        authority_list: Vec<Address>,
        db_path: &str,
    ) -> Self {
        Actuator {
            function,
            height,
            round,
            lock_round: None,
            lock_proposal: None,
            authority_list,
            proposal: Vec::new(),
            byzantine: byzantine_proposal(),
            storage: Storage::new(db_path),
            vote_cache: VoteCache::new(),
            stime: Timespec::new(0, 0),
            htime: Timespec::new(0, 0),
        }
    }

    /// A function to set a new authority list.
    pub fn set_authority_list(&mut self, authority_list: Vec<Address>) {
        self.authority_list = authority_list;
    }

    /// A function to do whitebox testing with test cases input.
    pub fn proc_test(&mut self, cases: BftTest) -> BftResult<()> {
        self.init();
        for case in cases.iter() {
            if case == &SHOULD_COMMIT {
                if let Some(commit) = self.function.try_get_commit() {
                    self.storage_msg(Msg::Commit(commit.clone()));
                    self.check_commit(commit)?;
                    let status = self.generate_status();
                    self.function.send(FrameSend::Status(status));
                    println!(
                        "Height {:?}, use time {:?}",
                        self.height,
                        time::get_time() - self.htime
                    );
                    self.goto_next_height();
                }
            } else if case == &NULL_ROUND {
                self.goto_next_round();
            } else if case == &NO_COMMIT_BUT_LOCK {
                self.lock_proposal = Some(self.proposal.clone());
                self.lock_round = Some(self.round);
                if self.function.try_get_commit().is_some() {
                    self.lock_proposal = Some(self.proposal.clone());
                    return Err(BftError::CommitInvalid(self.height));
                }
                self.goto_next_round();
            } else if case == &NO_COMMIT_NO_LOCK {
                if self.function.try_get_commit().is_some() {
                    return Err(BftError::CommitInvalid(self.height));
                }
                self.goto_next_round();
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
                    let proposal = self.generate_proposal(proposer, self.lock_round, Vec::new());
                    self.storage_msg(Msg::Proposal(proposal.clone()));
                    self.function.send(FrameSend::Proposal(proposal));
                } else {
                    panic!("Proposer index beyond authority list!");
                }
                self.generate_prevote(prevote);
                self.check_prevote()?;
                self.generate_precommit(precommit);
                self.check_precommit()?;
            }
        }
        println!("Total test time; {:?}", time::get_time() - self.stime);
        Ok(())
    }

    fn generate_feed(&self) -> Feed {
        let mut proposal = vec![0, 0, 0, 0, 0, 0];
        while self.byzantine.contains(&proposal) {
            let mut rng = thread_rng();
            for ii in proposal.iter_mut() {
                *ii = rng.gen();
            }
        }
        let res = Feed {
            height: self.height,
            proposal,
        };
        self.storage_msg(Msg::Feed(res.clone()));
        res
    }

    fn generate_status(&self) -> Status {
        let res = Status {
            height: self.height,
            authority_list: self.authority_list.clone(),
        };
        self.storage_msg(Msg::Status(res.clone()));
        res
    }

    fn generate_proposal(
        &mut self,
        auth_index: usize,
        lock_round: Option<u64>,
        lock_votes: Vec<Vote>,
    ) -> Proposal {
        let mut proposal = vec![0, 0, 0, 0, 0, 0];
        while self.byzantine.contains(&proposal) {
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

    fn generate_prevote(&mut self, prevote: Vec<u8>) {
        let proposal = if self.lock_proposal.is_none() {
            self.proposal.clone()
        } else {
            self.lock_proposal.clone().unwrap()
        };

        for i in 0..2 {
            if prevote[i] == NORMAL {
                let vote = Vote {
                    height: self.height,
                    round: self.round,
                    vote_type: VoteType::Prevote,
                    proposal: proposal.clone(),
                    voter: self.authority_list[i + 1].clone(),
                };

                self.storage_msg(Msg::Vote(vote.clone()));
                self.function.send(FrameSend::Vote(vote.clone()));
                self.vote_cache.add(vote);
            } else if prevote[i] == BYZANTINE {
                let vote = Vote {
                    height: self.height,
                    round: self.round,
                    vote_type: VoteType::Prevote,
                    proposal: self.byzantine[i].clone(),
                    voter: self.authority_list[i + 1].clone(),
                };

                self.storage_msg(Msg::Vote(vote.clone()));
                self.function.send(FrameSend::Vote(vote.clone()));
                self.vote_cache.add(vote);
            } else if prevote[i] == OFFLINE {
                return;
            } else {
                panic!("Invalid Test Case! {:?}", prevote);
            }
        }
    }

    fn generate_precommit(&mut self, precommit: Vec<u8>) {
        let proposal = if self.lock_proposal.is_none() {
            self.proposal.clone()
        } else {
            self.lock_proposal.clone().unwrap()
        };

        for i in 0..2 {
            if precommit[i] == NORMAL {
                let vote = Vote {
                    height: self.height,
                    round: self.round,
                    vote_type: VoteType::Precommit,
                    proposal: proposal.clone(),
                    voter: self.authority_list[i + 1].clone(),
                };

                self.storage_msg(Msg::Vote(vote.clone()));
                self.function.send(FrameSend::Vote(vote.clone()));
                self.vote_cache.add(vote);
            } else if precommit[i] == BYZANTINE {
                let vote = Vote {
                    height: self.height,
                    round: self.round,
                    vote_type: VoteType::Precommit,
                    proposal: self.byzantine[i].clone(),
                    voter: self.authority_list[i + 1].clone(),
                };

                self.storage_msg(Msg::Vote(vote.clone()));
                self.function.send(FrameSend::Vote(vote.clone()));
                self.vote_cache.add(vote);
            } else if precommit[i] == OFFLINE {
                return;
            } else {
                panic!("Invalid Test Case! {:?}", precommit);
            }
        }
    }

    fn check_prevote(&mut self) -> BftResult<()> {
        let vote = self.handle_vote(VoteType::Prevote)?;
        let mut clean_flag = true;

        if let Some(prevote_set) =
            self.vote_cache
                .get_voteset(self.height, self.height, VoteType::Prevote)
        {
            // check prevote condition
            for (p, count) in prevote_set.votes_by_proposal {
                if self.is_above_threshold(count).is_ok() {
                    clean_flag = false;
                    if !p.is_empty() {
                        self.set_polc(p);
                    } else {
                        self.clean_polc();
                    }
                }
            }
        } else {
            return Err(BftError::IllegalVote(vote));
        }
        if clean_flag {
            self.proposal = Vec::new();
        }
        Ok(())
    }

    fn check_precommit(&mut self) -> BftResult<()> {
        let vote = self.handle_vote(VoteType::Precommit)?;
        if let Some(prevote_set) =
            self.vote_cache
                .get_voteset(self.height, self.height, VoteType::Prevote)
        {
            // check precommit condition
            self.is_above_threshold(prevote_set.count)?;
            for (v, count) in prevote_set.votes_by_proposal.iter() {
                if self.is_above_threshold(*count).is_ok() {
                    if v != &vote.proposal {
                        return Err(BftError::PrecommitErr(self.height, self.round));
                    }

                    let polc = prevote_set.extract_polc(
                        self.height,
                        self.round,
                        VoteType::Prevote,
                        &vote.proposal.clone(),
                    );
                    if polc.len() < 3 {
                        return Err(BftError::PrecommitDiffPoLC(vote));
                    }
                }
            }
        } else {
            return Err(BftError::IllegalVote(vote));
        }
        Ok(())
    }

    fn check_commit(&mut self, commit: Commit) -> BftResult<()> {
        // TODO
        let hash = commit.result.clone();
        if hash != self.proposal {
            return Err(BftError::CommitIncorrect(self.height));
        }
        Ok(())
    }

    fn handle_vote(&mut self, vote_type: VoteType) -> BftResult<Vote> {
        let mut vote;
        match self.function.recv() {
            FrameRecv::Proposal(p) => return Err(BftError::AbnormalProposal(p)),
            FrameRecv::Vote(v) => vote = v,
        }
        if vote.vote_type == vote_type || self.byzantine.contains(&vote.proposal) {
            // check vote type and vote proposal
            return Err(BftError::IllegalVote(vote));
        }
        self.vote_cache.add(vote.clone());
        self.storage_msg(Msg::Vote(vote.clone()));
        Ok(vote)
    }

    fn is_above_threshold(&self, num: usize) -> BftResult<()> {
        if num * 3 <= self.authority_list.len() * 2 {
            return Err(BftError::ShouldNotPrecommit(self.height, self.round));
        }
        Ok(())
    }

    fn set_polc(&mut self, proposal: Vec<u8>) {
        self.proposal = proposal.clone();
        self.lock_round = Some(self.round);
        self.lock_proposal = Some(proposal);
    }

    fn clean_polc(&mut self) {
        self.proposal = Vec::new();
        self.lock_round = None;
        self.lock_proposal = None;
    }

    fn storage_msg(&self, msg: Msg) {
        let res = self.storage.insert(msg.clone());
        if res.is_err() {
            panic!("SQLite Error {:?} when insert {:?}", res, msg);
        }
    }

    fn goto_next_height(&mut self) {
        self.vote_cache.clear_prevote_count();
        self.lock_round = None;
        self.lock_proposal = None;
        self.proposal = Vec::new();
        self.round = 0;
        self.height += 1;
        self.htime = time::get_time();
    }

    fn goto_next_round(&mut self) {
        if self.lock_round.is_none() {
            self.proposal = Vec::new();
        } else {
            self.proposal = self.lock_proposal.clone().unwrap();
        }
        self.round += 1;
    }

    fn init(&mut self) {
        let init = self.generate_status();
        self.storage_msg(Msg::Status(init.clone()));
        self.function.send(FrameSend::Status(init));
        self.htime = time::get_time();
    }
}
