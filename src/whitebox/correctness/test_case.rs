use crate::whitebox::correctness::random::*;
use rand::random;

/// A basic test unit.
pub type BftTestUnit = [u8; 6];
/// A BFT test case.
pub type BftTest = Vec<BftTestUnit>;

pub(crate) const OFFLINE: u8 = 0;
pub(crate) const NORMAL: u8 = 1;
pub(crate) const BYZANTINE: u8 = 2;
pub(crate) const NULL_ROUND: [u8; 6] = [7, 7, 7, 7, 7, 7];
pub(crate) const SHOULD_COMMIT: [u8; 6] = [8, 8, 8, 8, 8, 8];
pub(crate) const NO_COMMIT_BUT_LOCK: [u8; 6] = [9, 9, 9, 8, 8, 8];
pub(crate) const NO_COMMIT_NO_LOCK: [u8; 6] = [9, 9, 9, 9, 9, 9];

pub(crate) fn byzantine_proposal() -> Vec<Vec<u8>> {
    vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1],
        vec![2, 2, 2, 2, 2, 2],
    ]
}

///
pub fn no_byzantine_cases() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..100 {
        cases.push([1, 1, 1, 1, 1, 1]);
        cases.push(SHOULD_COMMIT);
    }
    cases
}

///
pub fn one_offline_cases() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..100 {
        cases.push(rand_attribute(OFFLINE, NORMAL));
        cases.push(SHOULD_COMMIT);
    }
    cases
}

///
pub fn one_byzantine_cases() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..100 {
        cases.push(rand_attribute(BYZANTINE, NORMAL));
        cases.push(SHOULD_COMMIT);
    }
    cases
}

///
pub fn two_byzantine_cases() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..99 {
        cases.push(rand_two_attribute(BYZANTINE, NORMAL));
        cases.push(NO_COMMIT_NO_LOCK);
    }
    cases.push([1, 1, 1, 1, 1, 1]);
    cases.push(SHOULD_COMMIT);
    cases
}

///
pub fn two_offline_cases() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..10 {
        cases.push(rand_two_attribute(OFFLINE, NORMAL));
        cases.push(NO_COMMIT_NO_LOCK);
        cases.push(NULL_ROUND);
        cases.push(NULL_ROUND);
    }
    cases.push([1, 1, 1, 1, 1, 1]);
    cases.push(SHOULD_COMMIT);
    cases
}

///
pub fn two_byzantine_one_offline() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..10 {
        cases.push(rand_two_attribute(BYZANTINE, OFFLINE));
        cases.push(NO_COMMIT_NO_LOCK);
    }
    cases.push([1, 1, 1, 1, 1, 1]);
    cases.push(SHOULD_COMMIT);
    cases
}

///
pub fn round_leap() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..10 {
        for _ in 0..random::<u8>() {
            cases.push(rand_two_attribute(OFFLINE, NORMAL));
            cases.push(NO_COMMIT_NO_LOCK);
        }
        cases.push(rand_two_attribute(OFFLINE, NORMAL));
        cases.push(NO_COMMIT_NO_LOCK);
        cases.push([1, 1, 1, 1, 1, 1]);
        cases.push(SHOULD_COMMIT);
    }
    cases
}

///
pub fn lock_proposal() -> BftTest {
    let mut cases = Vec::new();
    for _ in 0..100 {
        if random::<bool>() {
            cases.push([1, 1, 1, 1, 0, 2]);
            cases.push(NO_COMMIT_BUT_LOCK);
        } else {
            cases.push([1, 2, 0, 1, 2, 0]);
            cases.push(NO_COMMIT_NO_LOCK);
        }
    }
    cases.push([1, 1, 1, 1, 1, 1]);
    cases.push(SHOULD_COMMIT);
    cases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_devide() {
        let unit: BftTestUnit = [1, 1, 1, 2, 2, 2];
        let cases: BftTest = vec![unit];
        for c in cases.iter() {
            let prevote = c[0..3].to_vec();
            let precommit = c[3..6].to_vec();
            assert_eq!(prevote, vec![1, 1, 1]);
            assert_eq!(precommit, vec![2, 2, 2]);
        }
    }
}
