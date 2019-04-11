use std::fmt;

#[derive(Debug)]
pub enum BftError {
    CommitDiff(u64),
    CommitIncorrect(u64),
    CommitInvalid(u64),
    MislaidCommit(u64),
    MultipleCommit(u64),
}

impl fmt::Display for BftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String = match *self {
            BftError::CommitDiff(height) => format!("Commit Different at Height {:?}", height),
            BftError::CommitIncorrect(height) => {
                format!("Commit Differ from Proposal at Height {:?}", height)
            }
            BftError::CommitInvalid(height) => format!("No Proposal at Height {:?}", height),
            BftError::MislaidCommit(height) => {
                format!("Mislaid Commit of Height {:?}", height)
            }
            BftError::MultipleCommit(height) => {
                format!("Multiple Commit at Height {:?}", height)
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
