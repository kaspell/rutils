use bvec::BitVec;


pub struct BloomFilter {
        e_storage: bvec::BitVec,
        bit_cnt: usize,
        hfn_cnt: usize,
}

impl BloomFilter {
        pub fn new(bit_cnt: usize, hfn_cnt: usize) -> Self {
                let bv = BitVec { blocks: vec![0u8; (bit_cnt/8)+1], size: bit_cnt };
                Self { e_storage: bv, bit_cnt, hfn_cnt }
        }

        pub fn add(self: &mut Self, s: &str) {
                self.e_storage.set(self.hash0(s));
                self.e_storage.set(self.hash1(s));
                if self.hfn_cnt < 3 {
                        return
                }
                for i in 3..=self.hfn_cnt {
                        self.e_storage.set((self.hash0(s).wrapping_add(self.hash1(s).wrapping_mul(i))) % self.bit_cnt);
                }
        }

        pub fn contains(self: &Self, s: &str) -> bool {
                if !self.e_storage.is_set(self.hash0(s)) || !self.e_storage.is_set(self.hash0(s)) {
                        return false;
                }
                if self.hfn_cnt < 3 {
                        return true;
                }
                for i in 3..=self.hfn_cnt {
                        if !self.e_storage.is_set((self.hash0(s).wrapping_add(self.hash1(s).wrapping_mul(i))) % self.bit_cnt) {
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
                h % self.bit_cnt
        }

        fn hash1(self: &Self, s: &str) -> usize {
                let mut h: usize = 0;
                for c in s.bytes() {
                        h = (((c as usize).wrapping_add(h << 6)).wrapping_add(h << 16)).wrapping_sub(h);
                }
                h % self.bit_cnt
        }
}