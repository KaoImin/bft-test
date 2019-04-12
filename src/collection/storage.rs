use crate::{collection::util::*, Commit, Feed, Proposal, Status, Vote};

use SQLite::{params, Connection, Result};

pub(crate) struct Storage(Connection);

pub(crate) enum Msg {
    Proposal(Proposal),
    Vote(Vote),
    Commit(Commit),
    Feed(Feed),
    Status(Status),
}

impl Storage {
    pub(crate) fn new(db_path: &str) -> Self {
        let conn = Connection::open(db_path).expect("Create SQLite failed!");
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
        conn.execute(
            "CREATE TABLE feed (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                proposal    TEXT NOT NULL,
            )",
            params![],
        )
        .expect("Create vote table failed!");
        conn.execute(
            "CREATE TABLE status (
                timestamp   TEXT PRIMARY KEY,
                height      INTEGER NOT NULL,
                authority   TEXT NOT NULL,
            )",
            params![],
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
                    "INSERT INTO commit (timestamp, height, commit)
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
