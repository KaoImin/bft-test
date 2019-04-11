use crate::{Commit, Proposal, Vote};
use serde_json::to_string;
use time::Timespec;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct StorageProposal {
    pub(crate) timestamp: Timespec,
    pub(crate) height: i64,
    pub(crate) round: i64,
    pub(crate) proposal: String,
}

impl StorageProposal {
    pub(crate) fn from_proposal(proposal: Proposal) -> Self {
        StorageProposal {
            timestamp: time::get_time(),
            height: proposal.height as i64,
            round: proposal.round as i64,
            proposal: to_string(&proposal).unwrap(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct StorageVote {
    pub(crate) timestamp: Timespec,
    pub(crate) height: i64,
    pub(crate) round: i64,
    pub(crate) voter: Vec<u8>,
    pub(crate) vote: String,
}

impl StorageVote {
    pub(crate) fn from_vote(vote: Vote) -> Self {
        StorageVote {
            timestamp: time::get_time(),
            height: vote.height as i64,
            round: vote.round as i64,
            voter: vote.voter.clone(),
            vote: to_string(&vote).unwrap(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct StorageCommit {
    pub(crate) timestamp: Timespec,
    pub(crate) height: i64,
    pub(crate) commit: String,
}

impl StorageCommit {
    pub(crate) fn from_commit(commit: Commit) -> Self {
        StorageCommit {
            timestamp: time::get_time(),
            height: commit.height as i64,
            commit: to_string(&commit).unwrap(),
        }
    }
}
