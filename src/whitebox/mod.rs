use crate::whitebox::error::{BftError, FrameError};
use serde_derive::{Serialize, Deserialize};

type Hash = Vec<u8>;
type Address = Vec<u8>;
/// BFT result.
pub type BftResult<T> = Result<T, BftError>;
/// Test framework result.
pub type FrameResult<T> = Result<T, FrameError>;

///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameRecv {
    ///
    Proposal(Proposal),
    ///
    Vote(Vote),
}

///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum FrameSend {
    ///
    Proposal(Proposal),
    ///
    Vote(Vote),
    ///
    Feed(Feed),
    ///
    Status(Status),
}

///
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Proposal {
    ///
    pub height: u64,
    ///
    pub round: u64,
    ///
    pub content: Hash,
    ///
    pub proposer: Address,
    ///
    pub lock_round: Option<u64>,
    ///
    pub lock_votes: Vec<Vote>,
}

///
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Vote {
    ///
    pub height: u64,
    ///
    pub round: u64,
    ///
    pub vote_type: VoteType,
    ///
    pub proposal: Hash,
    ///
    pub voter: Address,
}

///
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Commit {
    ///
    pub node: u8,
    ///
    pub height: u64,
    ///
    pub result: Vec<u8>,
}

///
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Feed {
    ///
    pub height: u64,
    ///
    pub proposal: Vec<u8>,
}

///
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Status {
    ///
    pub height: u64,
    ///
    pub authority_list: Vec<Address>,
}

///
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
pub enum VoteType {
    ///
    Prevote,
    ///
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
    fn cal_proposer(&self, height: u64, round: u64) -> usize;
}

///
pub mod actuator;
///
pub mod collection;
///
pub mod correctness;
///
pub mod error;
