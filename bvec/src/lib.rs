#[derive(Clone)]
pub struct BitVec {
        blocks: Vec::<u8>,
        size: usize,
}

impl BitVec {
        pub fn new() -> Self {
                Self { blocks: vec![0; 1], size: 0 }
        }

        pub fn with_capacity(sz: usize) -> Self {
                Self { blocks: Vec::<u8>::with_capacity(sz.div_ceil(8)), size: sz }
        }

        /// Map a bit vector index to the relevant indices of the implementation:
        /// the byte index and the bit index within that byte
        #[inline]
        fn get_idxs(self: &Self, i: usize) -> (usize, usize) {
                let mut byte_idx = i.div_ceil(8);
                if byte_idx != i/8 {
                        byte_idx -= 1;
                }
                if byte_idx > self.size {
                        panic!("Index out of bounds: size is {} but queried for {}", self.size, i);
                }
                (byte_idx, i % 8)
        }

        pub fn is_set(self: &Self, i: usize) -> bool {
                let (byte_idx, bit_idx) = self.get_idxs(i);
                ((1u8 << bit_idx) & self.blocks[byte_idx]) > 0
        }

        pub fn set(self: &mut Self, i: usize) {
                let (byte_idx, bit_idx) = self.get_idxs(i);
                self.blocks[byte_idx] |= 1u8 << bit_idx;
        }

        pub fn clear(self: &mut Self, i: usize) {
                let (byte_idx, bit_idx) = self.get_idxs(i);
                self.blocks[byte_idx] &= !(1u8 << bit_idx);
        }

        pub fn push_1(self: &mut Self) {
                if self.size != 0 && self.size % 8 == 0 {
                        self.blocks.push(0u8);
                }
                let (byte_idx, bit_idx) = self.get_idxs(self.size);
                self.blocks[byte_idx] |= 1u8 << bit_idx;
                self.size += 1;
        }

        pub fn push_0(self: &mut Self) {
                if self.size != 0 && self.size % 8 == 0 {
                        self.blocks.push(0u8);
                }
                let (byte_idx, bit_idx) = self.get_idxs(self.size);
                self.blocks[byte_idx] &= !(1u8 << bit_idx);
                self.size += 1;
        }
}