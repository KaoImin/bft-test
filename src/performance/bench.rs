use crate::{
    node::{FrameResult, Node, INIT_HEIGHT},
    Address, CommitInfo, Feed, Transmit,
};

use criterion;
use crossbeam::crossbeam_channel::{select, unbounded, Sender};
use rand::{thread_rng, Rng};

use std::{thread, time::Duration};

pub struct Benchmark<T> {
    node_0: Node<T>,
    node_1: Node<T>,
    node_2: Node<T>,
    node_3: Node<T>,
}

impl<T: Transmit + Clone + Send + 'static> Benchmark<T> {
    fn new(
        transmit: T,
        authority_list: Vec<Address>,
        send_0: Sender<CommitInfo>,
        send_1: Sender<CommitInfo>,
        send_2: Sender<CommitInfo>,
        send_3: Sender<CommitInfo>,
    ) -> Self {
        let node_0 = Node::new(
            transmit.clone(),
            0,
            authority_list[0].clone(),
            authority_list.clone(),
            send_0,
        );
        let node_1 = Node::new(
            transmit.clone(),
            1,
            authority_list[1].clone(),
            authority_list.clone(),
            send_1,
        );
        let node_2 = Node::new(
            transmit.clone(),
            2,
            authority_list[2].clone(),
            authority_list.clone(),
            send_2,
        );
        let node_3 = Node::new(
            transmit,
            3,
            authority_list[3].clone(),
            authority_list,
            send_3,
        );
        Benchmark {
            node_0,
            node_1,
            node_2,
            node_3,
        }
    }

    pub fn start(max_height: u64, transmit: T, authority_list: Vec<Address>) -> FrameResult<()> {
        let (send_0, recv_0) = unbounded();
        let (send_1, recv_1) = unbounded();
        let (send_2, recv_2) = unbounded();
        let (send_3, recv_3) = unbounded();

        let mut engine = Benchmark::new(transmit, authority_list, send_0, send_1, send_2, send_3);

        for i in INIT_HEIGHT..max_height + 1 {
            engine.node_0.set_feed(generate_feed(i))?;
            engine.node_1.set_feed(generate_feed(i))?;
            engine.node_2.set_feed(generate_feed(i))?;
            engine.node_3.set_feed(generate_feed(i))?;
        }

        thread::spawn(move || loop {
            select! {
                recv(recv_0) -> commit => {
                    if let Ok(c) = commit {
                        let height = c.height;
                        if height < max_height {
                            let _ = engine.node_0.set_status(height);
                            println!("Node 0 height{}, consensus time {:?}, total time{:?}",
                                c.height,
                                c.interval,
                                c.total_interval
                            );
                        } else {
                            let _ = engine.node_0.stop();
                        }
                    }
                }
                recv(recv_1) -> commit => {
                    if let Ok(c) = commit {
                        let height = c.height;
                        if height < max_height {
                            let _ = engine.node_1.set_status(height);
                            println!("Node 1 height{}, consensus time {:?}, total time{:?}",
                                c.height,
                                c.interval,
                                c.total_interval
                            );
                        } else {
                            let _ = engine.node_1.stop();
                        }
                    }
                }
                recv(recv_2) -> commit => {
                    if let Ok(c) = commit {
                        let height = c.height;
                        if height < max_height {
                            let _ = engine.node_2.set_status(height);
                            println!("Node 2 height{}, consensus time {:?}, total time{:?}",
                                c.height,
                                c.interval,
                                c.total_interval
                            );
                        } else {
                            let _ = engine.node_2.stop();
                        }
                    }
                }
                recv(recv_3) -> commit => {
                    if let Ok(c) = commit {
                        let height = c.height;
                        if height < max_height {
                            let _ = engine.node_3.set_status(height);
                            println!("Node 3 height{}, consensus time {:?}, total time{:?}",
                                c.height,
                                c.interval,
                                c.total_interval
                            );
                        } else {
                            let _ = engine.node_3.stop();
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

pub fn generate_feed(height: u64) -> Feed {
    let mut proposal = vec![1, 2, 3];
    let mut rng = thread_rng();

    for ii in proposal.iter_mut() {
        *ii = rng.gen();
    }
    Feed { height, proposal }
}
