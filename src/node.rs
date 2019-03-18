use crate::error::ConsensusError;
use crate::*;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::{thread::Thread, time::Instant};

const INIT_HEIGHT: usize = 0;

pub struct Node<T, F> {
    pub send: T,
    pub recv: T,
    pub id: u32,
    pub thread: Thread,
    pub address: F,
    pub authority_list: Vec<F>,
    pub transmit_channel: HashMap<u32, T>,
    pub height: usize,
    pub result: HashMap<usize, Vec<u8>>,
    pub htime: Instant,
}

impl<T, F> Node<T, F>
where
    T: Transmit,
    F: Clone + Eq + PartialEq,
{
    pub fn new(s: T, r: T, id: u32, node_thread: Thread, addr: F) -> Self {
        Node {
            send: s,
            recv: r,
            id,
            thread: node_thread,
            address: addr,
            authority_list: Vec::new(),
            transmit_channel: HashMap::new(),
            height: INIT_HEIGHT,
            result: HashMap::new(),
            htime: Instant::now(),
        }
    }

    pub fn set_authority_list(
        &mut self,
        authority_list: Vec<F>,
        transmit_channel: HashMap<u32, T>,
    ) {
        self.authority_list = authority_list;
        self.transmit_channel = transmit_channel;
    }

    fn save_commit(&mut self, commit: Commit) -> Result<(), ConsensusError> {
        if self.result.contains_key(&self.height) {
            return Err(ConsensusError::MultipleCommit(self.height));
        }

        self.result.insert(self.height, commit.result);
        Ok(())
    }

    fn transpond_message<U: Clone + Eq + PartialEq>(&self, msg: ProtocolSend<U>) {
        for channels in self.transmit_channel.values() {
            channels.send2others(msg.clone());
        }
    }
}

pub fn generate_proposal() -> Vec<u8> {
    let mut proposal = vec![1, 2, 3];
    let mut rng = thread_rng();

    for ii in proposal.iter_mut() {
        *ii = rng.gen();
    }
    proposal
}
