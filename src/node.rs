use crate::check::HeightCollector;
use crate::*;

use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::time::{Duration, Instant};

const INIT_HEIGHT: usize = 0;

pub struct Node<T, F> {
    pub send: T,
    pub recv: T,
    pub id: u32,
    pub thread: ::std::thread::Thread,
    pub address: F,
    pub authority_list: Vec<F>,
    pub transmit_channel: HashMap<u32, T>,
    pub height: usize,
    pub result: HeightCollector,
    pub htime: Instant,
}

impl<T, F> Node<T, F>
where
    T: Transmit,
    F: Clone + Eq + PartialEq,
{
    pub fn new(s: T, r: T, id: u32, node_thread: ::std::thread::Thread, addr: F) -> Self {
        Node {
            send: s,
            recv: r,
            id,
            thread: node_thread,
            address: addr,
            authority_list: Vec::new(),
            transmit_channel: HashMap::new(),
            height: INIT_HEIGHT,
            result: HeightCollector::default(),
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
}

pub fn generate_proposal() -> Vec<u8> {
    let mut proposal = vec![1, 2, 3];
    let mut rng = thread_rng();

    for ii in proposal.iter_mut() {
        *ii = rng.gen();
    }
    proposal
}
