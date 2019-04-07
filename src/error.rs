use std::fmt;

pub enum ConsensusError {
    CommitDiff(u64),
    CommitIncorrect(u64),
    CommitInvalid(u64),
    MislaidCommit(u64),
    MultipleCommit(u64),
}

impl fmt::Display for ConsensusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: String = match *self {
            ConsensusError::CommitDiff(height) => {
                format!("Commit Different at Height {:?}", height)
            }
            ConsensusError::CommitIncorrect(height) => {
                format!("Commit Differ from Proposal at Height {:?}", height)
            }
            ConsensusError::CommitInvalid(height) => format!("No Proposal at Height {:?}", height),
            ConsensusError::MislaidCommit(height) => {
                format!("Mislaid Commit of Height {:?}", height)
            }
            ConsensusError::MultipleCommit(height) => {
                format!("Multiple Commit at Height {:?}", height)
            }
        };
        f.write_fmt(format_args!("Consensus Error ({})", msg))
    }
}
