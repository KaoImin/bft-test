use crate::error::ConsensusError;
use crate::*;

use crossbeam::crossbeam_channel::Sender;

use std::collections::HashMap;
use std::{thread::Thread, time::Instant};

pub(crate) const INIT_HEIGHT: u64 = 0;
pub(crate) type FrameResult<T> = Result<T, ConsensusError>;

pub struct Node<T> {
    transmit: T,
    id: u32,
    address: Address,
    authority_list: Vec<Address>,
    height: u64,
    result: HashMap<u64, Vec<u8>>,
    stime: Instant,
    htime: Instant,

    send: Sender<CommitInfo>,
}

impl<T> Node<T>
where
    T: Transmit,
{
    pub fn new(
        transmit: T,
        id: u32,
        addr: Address,
        authority_list: Vec<Address>,
        send: Sender<CommitInfo>,
    ) -> Self {
        Node {
            transmit,
            id,
            address: addr,
            authority_list,
            height: INIT_HEIGHT,
            result: HashMap::new(),
            stime: Instant::now(),
            htime: Instant::now(),

            send,
        }
    }

    pub fn set_authority_list(&mut self, authority_list: Vec<Address>) {
        self.authority_list = authority_list;
    }

    pub fn save_commit(&mut self, commit: Commit) -> FrameResult<()> {
        if self.result.contains_key(&self.height) {
            return Err(ConsensusError::MultipleCommit(self.height));
        }
        self.result.entry(self.height).or_insert(commit.result);
        Ok(())
    }

    pub fn set_feed(&self, feed: Feed) -> FrameResult<()> {
        self.transmit.recv4frame(ProtocolRecv::Feed(feed))?;
        Ok(())
    }

    pub fn send_outside(&self, commit: Commit) -> FrameResult<()> {
        let mut commit = CommitInfo::from_commit(commit);
        commit.set_interval(self.htime);
        commit.set_total_interval(self.stime);
        self.send.send(commit).unwrap();
        Ok(())
    }

    pub fn set_status(&mut self, height: u64) -> FrameResult<()> {
        self.height = height + 1;
        let s = Status {
            height,
            authority_list: self.authority_list.clone(),
        };
        self.transmit.recv4frame(ProtocolRecv::Status(s))?;
        Ok(())
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }

    pub fn stop(&self) -> FrameResult<()> {
        self.transmit.stop()
    }
}
