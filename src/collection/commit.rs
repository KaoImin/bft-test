use crate::error::ConsensusError;
use crate::*;

use lru_cache::LruCache;

use std::collections::{HashMap, HashSet};

// height collector includes at most 20 commit collectors
// and commit result of each height
#[derive(Debug)]
pub struct HeightCommitCollector {
    pub height_commit_collector: LruCache<u64, CommitCollector>,
    pub height_result: HashMap<u64, Vec<u8>>,
}

impl Default for collection::commit::HeightCommitCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl HeightCommitCollector {
    pub fn new() -> Self {
        HeightCommitCollector {
            height_commit_collector: LruCache::new(20),
            height_result: HashMap::new(),
        }
    }

    pub fn add_commit(&mut self, commit: Commit) -> Result<(), ConsensusError> {
        let node_id = commit.node;
        let height = commit.height;
        let consequence = commit.result;

        if self.height_result.contains_key(&height) {
            if consequence != self.height_result[&height] {
                return Err(ConsensusError::CommitDiff(height));
            }
            if let Some(height_commit) = self.height_commit_collector.get_mut(&height) {
                if !height_commit.add(node_id, consequence) {
                    return Err(ConsensusError::MultipleCommit(height));
                }
            }
        } else {
            // this is the first commit of the height
            self.height_result.insert(height, consequence.clone());
            let mut commit_collector = CommitCollector::new();
            let _ = commit_collector.add(node_id, consequence);
            self.height_commit_collector
                .insert(height, commit_collector);
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct CommitCollector {
    commit_collector: Vec<Vec<u8>>,
    do_not_commit: HashSet<u8>,
}

impl CommitCollector {
    fn new() -> Self {
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
