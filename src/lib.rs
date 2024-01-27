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
    }

    #[test]
    fn cipher_test() {
        let sym_key = [0xC6D7A45B5BEBD507u64, 0x116C7D4AB5BE5D70];
        let original = [0x4e,0x65,0x63,0x65,0x73,0x73,0x69,0x74,0x79,0x20,0x6b,0x6e,0x6f,0x77,0x73,0x20,0x6e,0x6f,0x20,0x6c,0x61,0x77];
        let mut transit = original;
        let mut c1 = isaac::XorCipher::new(sym_key.into_iter());
        let mut c2 = isaac::XorCipher::new(sym_key.into_iter());
        c1.endec(&mut transit);
        c2.endec(&mut transit);
        assert_eq!(transit, original);
    }
}
