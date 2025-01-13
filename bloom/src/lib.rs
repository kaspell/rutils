use bvec::BitVec;


pub struct BloomFilter {
        data: bvec::BitVec,
        m: usize,
        k: usize,
}

impl BloomFilter {
        pub fn new(m: usize, k: usize) -> Self {
                let bv = BitVec { blocks: vec![0u8; (m/8)+1], size: m };
                Self { data: bv, m: m, k: k }
        }

        pub fn add(self: &mut Self, s: &str) {
                self.data.set(self.hash0(s));
                self.data.set(self.hash1(s));
                if self.k < 3 {
                        return
                }
                for i in 3..=self.k {
                        self.data.set((self.hash0(s).wrapping_add(self.hash1(s).wrapping_mul(i))) % self.m);
                }
        }

        pub fn contains(self: &Self, s: &str) -> bool {
                if !self.data.is_set(self.hash0(s)) || !self.data.is_set(self.hash0(s)) {
                        return false;
                }
                if self.k < 3 {
                        return true;
                }
                for i in 3..=self.k {
                        if !self.data.is_set((self.hash0(s).wrapping_add(self.hash1(s).wrapping_mul(i))) % self.m) {
                                return false;
                        }
                }
                return true;
        }

        fn hash0(self: &Self, s: &str) -> usize {
                let mut h: usize = 5381;
                for c in s.bytes() {
                        h = ((h << 5).wrapping_add(h)).wrapping_add(c as usize);
                }
                h % self.m
        }

        fn hash1(self: &Self, s: &str) -> usize {
                let mut h: usize = 0;
                for c in s.bytes() {
                        h = (((c as usize).wrapping_add(h << 6)).wrapping_add(h << 16)).wrapping_sub(h);
                }
                h % self.m
        }
}