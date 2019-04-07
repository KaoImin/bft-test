use crate::collection::{commit::HeightCommitCollector, proposal::ProposalCollector};
use crate::*;

use crossbeam::crossbeam_channel::{unbounded, Receiver, Sender};

struct VerifyNoede {
    send: Sender<ProtocolRecv>,
    recv: Receiver<ProtocolSend>,

    proposal_collect: ProposalCollector,
    commit_collect: HeightCommitCollector,
}
