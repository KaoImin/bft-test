use crate::*;
use std::fmt;

#[derive(Debug)]
pub enum BftError {
    CommitDiff(u64),
    CommitIncorrect(u64),
    CommitInvalid(u64),
    MislaidCommit(u64),
    MultipleCommit(u64),
    ShouldNotPrecommit(u64, u64),
    AbnormalProposal(Proposal),
    IllegalPrecommit(u64, u64),
    IllegalPrevote(u64, u64),
}

impl fmt::Display for BftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String = match self {
            BftError::CommitDiff(h) => format!("Commit Different at Height {:?}", h),
            BftError::CommitIncorrect(h) => {
                format!("Commit Differ from Proposal at Height {:?}", h)
            }
            BftError::CommitInvalid(h) => format!("No Proposal at Height {:?}", h),
            BftError::MislaidCommit(h) => format!("Mislaid Commit of Height {:?}", h),
            BftError::MultipleCommit(h) => format!("Multiple Commit at Height {:?}", h),
            BftError::ShouldNotPrecommit(h, r) => format!(
                "Do Precommit without +2/3 Prevotes at Height {:?}, Round {:?}",
                h, r
            ),
            BftError::AbnormalProposal(p) => format!("Abnormal Proposal Occur {:?}", p.clone()),
            BftError::IllegalPrecommit(h, r) => {
                format!("Illegal Precommit at Height {:?}, Round {:?}", h, r)
            }
            BftError::IllegalPrevote(h, r) => {
                format!("Illegal Prevote at Height {:?}, Round {:?}", h, r)
            }
        };
        f.write_fmt(format_args!("BFT Error ({})!", msg))
    }
}

#[derive(Debug)]
pub enum FrameError {
    SQLiteErr(usize),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String = match *self {
            FrameError::SQLiteErr(i) => format!("SQLite Error {:?}", i),
        };
        f.write_fmt(format_args!("Frame Error ({})I", msg))
    }
}
