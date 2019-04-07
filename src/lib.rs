extern crate criterion;
extern crate crossbeam;
extern crate lru_cache;
extern crate rand;

pub mod collection;
pub mod correctness;
pub mod error;
pub mod node;
pub mod performance;
pub mod verify;

type Hash = Vec<u8>;
type Address = Vec<u8>;

use crate::node::FrameResult;
use std::time::{Duration, Instant};

#[derive(Clone, PartialEq, Eq)]
pub enum ProtocolSend {
    Proposal(Proposal),
    Vote(Vote),
    Commit(Commit),
}

#[derive(Clone, PartialEq, Eq)]
pub enum ProtocolRecv {
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
    pub lock_votes: Option<Vec<Vote>>,
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
pub struct CommitInfo {
    pub node: u8,
    pub height: u64,
    pub result: Vec<u8>,
    pub interval: Duration,
    pub total_interval: Duration,
}

impl CommitInfo {
    pub(crate) fn from_commit(commit: Commit) -> Self {
        CommitInfo {
            node: commit.node,
            height: commit.height,
            result: commit.result,
            interval: Duration::from_secs(0),
            total_interval: Duration::from_secs(0),
        }
    }

    pub(crate) fn set_interval(&mut self, start: Instant) {
        self.interval = Instant::now() - start;
    }

    pub(crate) fn set_total_interval(&mut self, start: Instant) {
        self.total_interval = Instant::now() - start;
    }
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
pub trait Transmit {
    ///
    fn recv4frame(&self, msg: ProtocolRecv) -> FrameResult<()>;
    ///
    fn send2frame(&self, msg: ProtocolSend) -> FrameResult<()>;
    // ///
    // fn frame_recv(&self) -> FrameResult<ProtocolSend>;
    ///
    fn stop(&self) -> FrameResult<()>;
}
