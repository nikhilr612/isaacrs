//! A no-std pure-rust implementation of the ISAAC pseudo-random number generator.
#![no_std]

pub mod isaac;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut prng = isaac::Isaac::default();
        let rf = prng.randf();
        assert!((0.0..=1.0).contains(&rf));
    }

    #[test]
    fn cipher_test() {
        todo!("More shall be added.")
    }
}
