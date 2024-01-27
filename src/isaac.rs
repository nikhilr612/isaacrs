use core::num::Wrapping;

const RNDSIZL: usize = 8;
const RANDSIZ: usize = 1 << RNDSIZL;
const BITMASK: usize = RANDSIZ-1;

pub struct Isaac {
	// Memory
	mem: [Wrapping<u64>; RANDSIZ],
	// Sequence of Results
	rsl: [Wrapping<u64>; RANDSIZ],
	counter: usize,
	// Accumulator and counter.
	aa: Wrapping<u64>,
	cc: Wrapping<u64>,
	// Previous result
	bb: Wrapping<u64>,
}

macro_rules! mix {
	($a: ident, $b: ident, $c: ident, $d: ident, $e: ident, $f: ident, $g: ident, $h: ident) => {
		$a-=$e; $f^=$h>>9;  $h+=$a; 
		$b-=$f; $g^=$a<<9;  $a+=$b; 
		$c-=$g; $h^=$b>>23; $b+=$c; 
		$d-=$h; $a^=$c<<15; $c+=$d; 
		$e-=$a; $b^=$d>>14; $d+=$e; 
		$f-=$b; $c^=$e<<20; $e+=$f; 
		$g-=$c; $d^=$f>>17; $f+=$g; 
		$h-=$d; $e^=$g<<14; $g+=$h;
	}
}

macro_rules! rot {
	(($a: expr) << $b: literal) => {
		(($a << $b) | ($a >> (64-$b)))
	};
	(($a: expr) >> $b: literal) => {
        (($a >> $b) | ($a << (64-$b)))
    };
}

impl Default for Isaac {
    fn default() -> Self {
        let mut ret = Isaac {
			mem: [Wrapping(0); RANDSIZ],
			rsl: [Wrapping(0); RANDSIZ],
			counter: 0,
			aa: Wrapping(0),
			bb: Wrapping(0),
			cc: Wrapping(0)
		};
		ret._init_routine(false);
		ret
    }
}

impl Isaac {
	
	pub fn with_seed(it: impl Iterator<Item=u64>) -> Isaac {
		let mut rsl = [Wrapping(0); RANDSIZ];
		for (c, val) in it.enumerate() {
			rsl[c % RANDSIZ] ^= val;
		}
		let mut ret = Isaac {
			mem: [Wrapping(0); RANDSIZ],
			rsl,
			counter: 0,
			aa: Wrapping(0),
			bb: Wrapping(0),
			cc: Wrapping(0)
		};
		ret._init_routine(true);
		ret
	}

	fn rngstep(&mut self, val: Wrapping<u64>, i: usize, j: usize) {
		let x = self.mem[i];
		self.aa = val;
		self.aa += self.mem[j];
		let y = (self.mem[((x.0 as usize) & BITMASK) >> 2] ^ self.aa) + (self.bb);
		self.mem[i] = y;
		self.bb = self.mem[((y.0 as usize >> RNDSIZL) & BITMASK) >> 2] + x;
		self.rsl[i] = self.bb;
	}

	fn gen_bulk(&mut self){
		self.cc += 1;
		self.bb += self.cc;
		let hsize = RANDSIZ/2;
		
		let mut i = 0;
		let mut j = hsize;
		
		while i < hsize {
			self.rngstep(!(self.aa ^ rot!((self.aa) << 21)), i, j); i += 1; j += 1;
			self.rngstep(  self.aa ^ rot!((self.aa) >> 5 ) , i, j); i += 1; j += 1;
			self.rngstep(  self.aa ^ rot!((self.aa) << 12) , i, j); i += 1; j += 1;
			self.rngstep(  self.aa ^ rot!((self.aa) >> 33) , i, j); i += 1; j += 1;
		}

		let mut j = 0;
		while j < hsize {
			self.rngstep(!(self.aa ^ rot!((self.aa) << 21)), i, j); i += 1; j += 1;
			self.rngstep(  self.aa ^ rot!((self.aa) >> 5 ) , i, j); i += 1; j += 1;
			self.rngstep(  self.aa ^ rot!((self.aa) << 12) , i, j); i += 1; j += 1;
			self.rngstep(  self.aa ^ rot!((self.aa) >> 33) , i, j); i += 1; j += 1;
		}
	}

	fn next_raw_u64(&mut self) -> u64 {
		if self.counter == 0 {
			self.gen_bulk();
			self.counter = RANDSIZ;
		}
		self.counter -= 1;
		self.rsl[self.counter].0
	}

	/// Random floating point number from 0 to 1
	pub fn randf(&mut self) -> f64 {
		(self.next_raw_u64() as f64) / (u64::MAX as f64)
	}

	/// Random floating point number in [a,b]
	pub fn uniform(&mut self, a: f64, b: f64) -> f64 {
		(b-a)*self.randf() + a
	}

	fn _init_routine(&mut self, flag: bool) {
		let mut a; let mut b; let mut c; let mut d; let mut e; let mut f; let mut g; let mut h;
		h = Wrapping(0x9e3779b9u64);
		a = h;
		b = h;
		c = h;
		d = h;
		e = h;
		f = h;
		g = h;
		for _i in 0..4 {
			// Scramble
			mix!(a,b,c,d,e,f,g,h);
		}
		for i in (0..RANDSIZ).step_by(8) {
			if flag {
				a+=self.rsl[i  ]; b+=self.rsl[i+1]; c+=self.rsl[i+2]; d+=self.rsl[i+3];
       			e+=self.rsl[i+4]; f+=self.rsl[i+5]; g+=self.rsl[i+6]; h+=self.rsl[i+7];
			}
			// Mix
			mix!(a,b,c,d,e,f,g,h);
			self.mem[i  ]=a; self.mem[i+1]=b; self.mem[i+2]=c; self.mem[i+3]=d;
   			self.mem[i+4]=e; self.mem[i+5]=f; self.mem[i+6]=g; self.mem[i+7]=h;
		}
		// Second pass
		if flag {
		    for i in (0..RANDSIZ).step_by(8){
		    	a+=self.mem[i  ]; b+=self.mem[i+1]; c+=self.mem[i+2]; d+=self.mem[i+3];
		    	e+=self.mem[i+4]; f+=self.mem[i+5]; g+=self.mem[i+6]; h+=self.mem[i+7];
		    	mix!(a,b,c,d,e,f,g,h);
		    	self.mem[i  ]=a; self.mem[i+1]=b; self.mem[i+2]=c; self.mem[i+3]=d;
		    	self.mem[i+4]=e; self.mem[i+5]=f; self.mem[i+6]=g; self.mem[i+7]=h;
		    }
		}
		self.gen_bulk();
	}
}

pub struct XorCipher {
	rng: Isaac,
	key: u64,
	mask: u64,
	count: usize
}

impl XorCipher {

	/// Create a new XOR Cipher that uses Isaac to generate key bits, and is seeded by `it`.
	pub fn new(it: impl Iterator<Item=u64>) -> XorCipher {
		let mut rng = Isaac::with_seed(it);
		let key = rng.next_raw_u64();
		XorCipher {
			rng,
			key,
			mask: 0xff,
			count: 0
		}
	}

	/// Generate a byte and xor it with provided byte.
	pub fn op(&mut self, byte: u8) -> u8 {
		let k1 = ((self.key & self.mask) >> (self.count*8)) as u8;
		self.count += 1;
		if self.count == 8 {
			self.count = 0;
			self.key = self.rng.next_raw_u64();
			self.mask = 0xff;
		} else {
			self.mask <<= 8;
		}
		byte ^ k1
	}

	pub fn endec(&mut self, data: &mut [u8]) {
		for val in data {
			*val = self.op(*val);
		}
	}
}