use crate::whitebox::collection::util::*;
use SQLite::{params, Connection, Result, NO_PARAMS};

pub(crate) struct Storage(Connection);

impl Storage {
    pub(crate) fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Create SQLite failed!");
        conn.execute(
            "CREATE TABLE proposal (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                round       INTEGER NOT NULL,
                proposal    TEXT NOT NULL
            )",
            NO_PARAMS,
        )
        .expect("Create proposal table failed!");
        conn.execute(
            "CREATE TABLE vote (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                round       INTEGER NOT NULL,
                voter       BLOB NOT NULL,
                vote        TEXT NOT NULL
            )",
            NO_PARAMS,
        )
        .expect("Create vote table failed!");
        conn.execute(
            "CREATE TABLE cmt (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                cmt         TEXT NOT NULL
            )",
            NO_PARAMS,
        )
        .expect("Create commit table failed!");
        conn.execute(
            "CREATE TABLE feed (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                feed        TEXT NOT NULL
            )",
            NO_PARAMS,
        )
        .expect("Create vote table failed!");
        conn.execute(
            "CREATE TABLE status (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                status      TEXT NOT NULL
            )",
            NO_PARAMS,
        )
        .expect("Create vote table failed!");

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
                    "INSERT INTO cmt (timestamp, height, cmt)
                        VALUES (?1, ?2, ?3)",
                    params![c.timestamp, c.height, c.commit],
                )?;
            }
            Msg::Feed(f) => {
                let f = StorageFeed::from_feed(f);
                self.0.execute(
                    "INSERT INTO feed (timestamp, height, feed)
                        VALUES (?1, ?2, ?3)",
                    params![f.timestamp, f.height, f.proposal],
                )?;
            }
            Msg::Status(s) => {
                let s = StorageStatus::from_status(s);
                self.0.execute(
                    "INSERT INTO status (timestamp, height, status)
                        VALUES (?1, ?2, ?3)",
                    params![s.timestamp, s.height, s.authority],
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::whitebox::{collection::util::Msg, *};
    use rand::random;

    fn generate_kb() -> Vec<u8> {
        (0..1024).map(|_| random::<u8>()).collect()
    }

    fn generate_addr() -> Vec<u8> {
        (0..160).map(|_| random::<u8>()).collect()
    }

    fn generate_msg() -> Vec<Msg> {
        let mut msg = Vec::new();
        msg.push(Msg::Proposal(Proposal {
            height: 1,
            round: 0,
            content: generate_kb(),
            proposer: generate_addr(),
            lock_round: None,
            lock_votes: Vec::new(),
        }));
        let prevote = Vote {
            height: 1,
            round: 0,
            vote_type: VoteType::Prevote,
            proposal: generate_kb(),
            voter: generate_addr(),
        };
        msg.push(Msg::Vote(prevote.clone()));
        let precommit = Vote {
            height: 1,
            round: 0,
            vote_type: VoteType::Precommit,
            proposal: generate_kb(),
            voter: generate_addr(),
        };
        msg.push(Msg::Vote(precommit.clone()));
        msg.push(Msg::Proposal(Proposal {
            height: 1,
            round: 2,
            content: generate_kb(),
            proposer: generate_addr(),
            lock_round: Some(1),
            lock_votes: vec![prevote.clone(), prevote.clone(), prevote.clone()],
        }));
        msg.push(Msg::Feed(Feed {
            height: 2,
            proposal: generate_kb(),
        }));
        msg.push(Msg::Commit(Commit {
            node: 0,
            height: 2,
            result: generate_kb(),
        }));
        msg.push(Msg::Status(Status {
            height: 3,
            authority_list: vec![
                generate_addr(),
                generate_addr(),
                generate_addr(),
                generate_addr(),
            ],
        }));
        msg
    }

    #[test]
    fn test_db() {
        let conn = Storage::new("db/test.db");
        let message = generate_msg();
        for msg in message.into_iter() {
            let res = conn.insert(msg.clone());
            if res.is_err() {
                panic!("SQLite error {:?}", res);
            }
        }
    }
}
