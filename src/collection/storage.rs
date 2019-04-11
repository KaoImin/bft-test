use crate::{collection::util::*, Commit, Proposal, Vote};

use SQLite::{params, Connection, Result};

pub(crate) struct Storage(Connection);

pub(crate) enum Msg {
    Proposal(Proposal),
    Vote(Vote),
    Commit(Commit),
}

impl Storage {
    pub(crate) fn new() -> Self {
        let conn = Connection::open_in_memory().expect("Create SQLite failed!");
        conn.execute(
            "CREATE TABLE proposal (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                round       INTEGER NOT NULL,
                proposal    TEXT NOT NULL,
            )",
            params![],
        )
        .expect("Create proposal table failed!");
        conn.execute(
            "CREATE TABLE vote (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                round       INTEGER NOT NULL,
                voter       BLOB NOT NULL,
                vote        TEXT NOT NULL,
            )",
            params![],
        )
        .expect("Create vote table failed!");
        conn.execute(
            "CREATE TABLE commit (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                commit      TEXT NOT NULL,
            )",
            params![],
        )
        .expect("Create commit table failed!");

        Storage(conn)
    }

    pub(crate) fn insert(&self, msg: Msg) -> Result<()> {
        match msg {
            Msg::Proposal(p) => {
                let p = StorageProposal::from_proposal(p);
                self.0.execute(
                    "INSERT INTO proposal (timestamp, height, round, proposal)
                        VALUES (?1, ?2, ?3, ?4)",
                    params![p.timestamp, p.height, p.round, p.proposal],
                )?;
            }
            Msg::Vote(v) => {
                let v = StorageVote::from_vote(v);
                self.0.execute(
                    "INSERT INTO vote (timestamp, height, round, voter, vote)
                        VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![v.timestamp, v.height, v.round, v.voter, v.vote],
                )?;
            }
            Msg::Commit(c) => {
                let c = StorageCommit::from_commit(c);
                self.0.execute(
                    "INSERT INTO commit (timestamp, height, commit)
                        VALUES (?1, ?2, ?3)",
                    params![c.timestamp, c.height, c.commit],
                )?;
            }
        }
        Ok(())
    }
}

// pub struct ProposalCollector {
//     pub proposals: LruCache<u64, ProposalRoundCollector>,
// }

// impl Default for ProposalCollector {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl ProposalCollector {
//     pub fn new() -> Self {
//         ProposalCollector {
//             proposals: LruCache::new(20),
//         }
//     }

//     pub fn add(&mut self, proposal: Proposal) -> bool {
//         let height = proposal.clone().height;
//         let round = proposal.clone().round;

//         if self.proposals.contains_key(&height) {
//             self.proposals
//                 .get_mut(&height)
//                 .unwrap()
//                 .add(round, proposal)
//         } else {
//             let mut round_proposals = ProposalRoundCollector::new();
//             round_proposals.add(round, proposal);
//             self.proposals.insert(height, round_proposals);
//             true
//         }
//     }

//     pub fn get_proposal(&mut self, height: u64, round: u64) -> Option<Proposal> {
//         self.proposals
//             .get_mut(&height)
//             .and_then(|prop| prop.get_proposal(round))
//     }
// }

// pub struct ProposalRoundCollector {
//     pub round_proposals: LruCache<u64, Proposal>,
// }

// impl Default for collection::proposal::ProposalRoundCollector {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl ProposalRoundCollector {
//     pub fn new() -> Self {
//         ProposalRoundCollector {
//             round_proposals: LruCache::new(20),
//         }
//     }

//     pub fn add(&mut self, round: u64, proposal: Proposal) -> bool {
//         if self.round_proposals.contains_key(&round) {
//             false
//         } else {
//             self.round_proposals.insert(round, proposal);
//             true
//         }
//     }

//     pub fn get_proposal(&mut self, round: u64) -> Option<Proposal> {
//         self.round_proposals.get_mut(&round).cloned()
//     }
// }
