use crate::correctness::bft_test::BftTestUnit;
use rand::{thread_rng, Rng};

pub(crate) fn rand_attribute(attri: u8, base: u8) -> BftTestUnit {
    let mut rng = thread_rng();
    let index_1: usize = rng.gen_range(0, 3);
    let index_2: usize = rng.gen_range(3, 6);
    let mut unit = [base; 6];
    for i in 0..6 {
        if i == index_1 || i == index_2 {
            unit[i] = attri;
        }
    }
    unit
}

pub(crate) fn rand_two_attribute(attri: u8, base: u8) -> BftTestUnit {
    let mut rng = thread_rng();
    let index_1: usize = rng.gen_range(0, 3);
    let index_2: usize = rng.gen_range(3, 6);
    let mut unit = [base; 6];
    for i in 0..6 {
        if i != index_1 || i != index_2 {
            unit[i] = attri;
        }
    }
    unit
}
