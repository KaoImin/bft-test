extern crate criterion;
extern crate crossbeam;
extern crate lru_cache;
extern crate rand;

pub mod check;
pub mod error;
pub mod node;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Proposal<T> {
    pub height: usize,
    pub round: usize,
    pub content: T,
    pub proposer: T,
    pub lock_round: Option<usize>,
    pub lock_votes: Option<Vec<Vote<T>>>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Vote<T> {
    pub height: usize,
    pub round: usize,
    pub vote_type: VoteType,
    pub proposal: T,
    pub voter: T,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Commit {
    pub node: u8,
    pub height: usize,
    pub result: Vec<u8>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Feed {
    pub height: usize,
    pub proposal: Vec<u8>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Status<T> {
    pub height: usize,
    pub authority_list: Vec<T>,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum VoteType {
    Prevote,
    Precommit,
}

pub trait TransformProposal {
    fn transform_proposal<T: Clone + Eq + PartialEq>(&self) -> Proposal<T>;
}

pub trait TransformVote {
    fn transform_vote<T: Clone + Eq + PartialEq>(&self) -> Vote<T>;
}

pub trait TransformCommit {
    fn transform_commit(&self) -> Commit;
}

pub trait TransformFeed {
    fn transform_feed(feed: Feed) -> Self;
}

pub trait TransformStatus {
    fn transform_status<T: Clone + Eq + PartialEq>(status: Status<T>) -> Self;
}

pub trait Transmit {
    fn send(&self);
    fn recv(&self);
}
