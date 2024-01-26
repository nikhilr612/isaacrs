# isaacrs
ISAAC (Indirection, Shift, Accumulate, Add, and Count) is a relatively fast and reasonably secure pseudorandom number generator (PRNG) developed by R. Jenkins Jr.
The expected cycle length of ISAAC is 2^(8295). `isaac-rs` is a pure-rust, no-std implementation of ISAAC+, a modified algorithm that eliminates bad states, as specified by J.P.Aumasson (2006), that uses bit rotations instead of bit shifts.
ISAAC is cryptographically secure in the sense that it is not known to be unsafe. This ought to be attributed partially to the dearth of rigorous cryptanalysis and stress-tests that algorithms like AES are subject to. Another cipher, RC4 (Ron's Code), a not-so-distant cousin of ISAAC, is now deemed unsuitable for cryptographic purposes.
Any use of ISAAC must keep this in mind.