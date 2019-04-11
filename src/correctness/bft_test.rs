use crate::correctness::random::*;
///
pub type BftTestUnit = [u8; 6];
///
pub type BftTest = Vec<BftTestUnit>;

pub(crate) const OFFLINE: u8 = 0;
pub(crate) const NORMAL: u8 = 1;
pub(crate) const BYZANTINE: u8 = 2;
pub(crate) const SHOULD_COMMIT: [u8; 6] = [8, 8, 8, 8, 8, 8];
pub(crate) const NO_COMMIT_BUT_LOCK: [u8; 6] = [9, 9, 9, 8, 8, 8];
pub(crate) const NO_COMMIT_NO_LOCK: [u8; 6] = [9, 9, 9, 9, 9, 9];

pub fn no_byzantine_cases() -> BftTest {
    let mut cases = Vec::new();
    for _i in 0..100 {
        cases.push([1, 1, 1, 1, 1, 1]);
        cases.push(SHOULD_COMMIT);
    }
    cases
}

pub fn one_offline_cases() -> BftTest {
    let mut cases = Vec::new();
    for _i in 0..100 {
        cases.push(rand_attribute(OFFLINE));
        cases.push(SHOULD_COMMIT);
    }
    cases
}

pub fn one_byzantine_cases() -> BftTest {
    let mut cases = Vec::new();
    for _i in 0..100 {
        cases.push(rand_attribute(BYZANTINE));
        cases.push(SHOULD_COMMIT);
    }
    cases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
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
