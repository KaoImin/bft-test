#![allow(unused_imports)]
#![allow(unused_results)]

extern crate criterion;
extern crate crossbeam;
extern crate lru_cache;
extern crate rand;

pub mod check;
pub mod error;
pub mod generate;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Commit {
    pub node: u8,
    pub height: usize,
    pub result: Vec<u8>,
}

pub trait Transmit {
    fn send(&self);
    fn recv(&self);
}
