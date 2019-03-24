use crate::collection::{commit::HeightCommitCollector, proposal::ProposalCollector};
use crate::*;

use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};

struct VerifyNoede<T> {
    send: Sender<ProtocolRecv<T>>,
    recv: Receiver<ProtocolSend<T>>,

    proposal_collect: ProposalCollector<T>,
    commit_collect: HeightCommitCollector,
}
