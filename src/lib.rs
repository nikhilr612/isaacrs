//! A no-std pure-rust implementation of the ISAAC pseudo-random number generator.
// #![no_std]

pub mod isaac;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut prng = isaac::Isaac::default();
        let rf = prng.randf();
        assert!((0.0..=1.0).contains(&rf));
        let rf = prng.uniform(1.0, 10.0);
        assert!((1.0..=10.0).contains(&rf));
    }

    #[test]
    fn test_with_seed() {
        let seed: [u64; 2] = [0xC6_D7_A4_5B_5B_EB_D5_07, 0x11];
        let mut prng = isaac::Isaac::with_seed(seed.into_iter());
        let mut other_array = [0.0; 1024];
        for elm in other_array.iter_mut() {
            *elm = prng.randf();
        }
        println!("{other_array:?}");
    }

    
    fn cipher_test() {
        todo!("More shall be added.")
    }
}
