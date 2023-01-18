#![feature(test)]
extern crate test;
use rand::{rngs::StdRng, Rng, SeedableRng};

#[cfg(test)]
mod bitvec;

#[cfg(test)]
mod vec;

#[cfg(test)]
mod vecbool;

pub fn bench_values(size: usize) -> Vec<bool> {
    (0..size).map(|n| n % 3 == 0).collect()
}

/// Generate random indexes for an [Iterator] with length equals to `len`
pub fn bench_random_access(len: usize) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(0);

    (0..len).map(|_| rng.gen_range(0..len)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_access() {
        const LEN: usize = 5;
        let rand_indexes_1 = bench_random_access(LEN);
        let rand_indexes_2 = bench_random_access(LEN);

        assert_eq!(rand_indexes_1, rand_indexes_2);
        assert_eq!(rand_indexes_1.len(), LEN);

        for index in rand_indexes_1.iter() {
            assert!(*index < LEN);
        }
    }
}
