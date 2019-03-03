use super::*;
use crate::error::ConsensusError;

use lru_cache::LruCache;

use std::collections::{HashMap, HashSet};
use std::str;

// height collector includes at most 20 commit collectors
// and commit result of each height
#[derive(Debug)]
pub struct HeightCollector {
    pub height_collector: LruCache<usize, CommitCollector>,
    pub height_result: HashMap<usize, Vec<u8>>,
}

impl Default for check::HeightCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl HeightCollector {
    pub fn new() -> Self {
        HeightCollector {
            height_collector: LruCache::new(20),
            height_result: HashMap::new(),
        }
    }

    pub fn add(&mut self, commit: Commit) -> Result<(), ConsensusError> {
        let node_id = commit.node;
        let height = commit.height;
        let consequence = commit.result;

        if self.height_result.contains_key(&height) {
            if consequence != self.height_result[&height] {
                return Err(ConsensusError::CommitDiff(height));
            }
            if let Some(a) = self.height_collector.get_mut(&height) {
                if !a.add(node_id, consequence) {
                    return Err(ConsensusError::MultipleCommit(height));
                }
            }
        } else {
            // this is the first commit of the height
            self.height_result.insert(height, consequence.clone());
            let mut commit_collector = CommitCollector::new();
            let _ = commit_collector.add(node_id, consequence);
            self.height_collector.insert(height, commit_collector);
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct CommitCollector {
    pub commit_collector: Vec<Vec<u8>>,
    pub do_not_commit: HashSet<u8>,
}

impl CommitCollector {
    pub fn new() -> Self {
        let mut commit_node: HashSet<u8> = HashSet::new();
        commit_node.insert(0);
        commit_node.insert(1);
        commit_node.insert(2);
        commit_node.insert(3);

        CommitCollector {
            commit_collector: Vec::new(),
            do_not_commit: commit_node,
        }
    }

    fn add(&mut self, node_id: u8, consequence: Vec<u8>) -> bool {
        if self.do_not_commit.contains(&node_id) {
            self.commit_collector.push(consequence);
            self.do_not_commit.remove(&node_id);
            return true;
        }
        false
    }
}
