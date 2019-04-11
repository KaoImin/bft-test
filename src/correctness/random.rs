use crate::correctness::bft_test::{BftTestUnit, NORMAL};
use rand::{thread_rng, Rng};

pub(crate) fn rand_attribute(attri: u8) -> BftTestUnit {
    let mut rng = thread_rng();
    let index_1: usize = rng.gen_range(0, 3);
    let index_2: usize = rng.gen_range(3, 6);
    let mut unit = [NORMAL; 6];
    for i in 0..6 {
        if i == index_1 || i == index_2 {
            unit[i] = attri;
        }
    }
    unit
}
