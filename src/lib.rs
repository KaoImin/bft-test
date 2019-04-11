extern crate criterion;
extern crate crossbeam;
extern crate lru_cache;
extern crate rand;

use crate::error::BftError;
use std::time::{Duration, Instant};

pub mod actuator;
pub mod collection;
pub mod correctness;
pub mod error;

type Hash = Vec<u8>;
type Address = Vec<u8>;
pub(crate) type BftResult<T> = Result<T, BftError>;

#[derive(Clone, PartialEq, Eq)]
pub enum FrameRecv {
    Proposal(Proposal),
    Vote(Vote),
}

#[derive(Clone, PartialEq, Eq)]
pub enum FrameSend {
    Proposal(Proposal),
    Vote(Vote),
    Feed(Feed),
    Status(Status),
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Proposal {
    pub height: u64,
    pub round: u64,
    pub content: Hash,
    pub proposer: Address,
    pub lock_round: Option<u64>,
    pub lock_votes: Vec<Vote>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Vote {
    pub height: u64,
    pub round: u64,
    pub vote_type: VoteType,
    pub proposal: Hash,
    pub voter: Address,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Commit {
    pub node: u8,
    pub height: u64,
    pub result: Vec<u8>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Feed {
    pub height: u64,
    pub proposal: Vec<u8>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Status {
    pub height: u64,
    pub authority_list: Vec<Address>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum VoteType {
    Prevote,
    Precommit,
}

///
pub trait Support {
    ///
    fn send(&self, msg: FrameSend);
    ///
    fn recv(&self) -> FrameRecv;
    ///
    fn try_get_commit(&self) -> Option<Commit>;
    ///
    fn stop(&self);
    ///
    #[inline(always)]
    fn cal_proposer(&self, height: u64, round: u64) -> usize;
}
