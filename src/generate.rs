use rand::{thread_rng, Rng};

use std::thread;
use std::time::{Duration, Instant};

const INIT_HEIGHT: usize = 1;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: u8,
    pub thread: thread::Thread,
    pub address: Option<Vec<u8>>,
    pub height: usize,
    pub htime: Instant,
}

impl Node {
    pub fn new(id: u8, node_thread: thread::Thread) -> Self {
        Node {
            id,
            thread: node_thread,
            address: None,
            height: INIT_HEIGHT,
            htime: Instant::now(),
        }
    }

    pub fn set_address(&mut self, addr: Vec<u8>) {
        self.address = Some(addr);
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
