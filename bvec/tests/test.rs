use bvec::*;


#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_bit_push_set() {
                let mut bv = BitVec::new();
                bv.push_1();
                bv.push_1();
                bv.push_0();
                bv.push_1();
                bv.push_0();
                bv.push_0();
                bv.push_1();
                bv.push_0();
                bv.push_0();
                bv.push_0();
                bv.push_0();
                bv.push_1();
                bv.push_0();
                bv.push_0();
                bv.push_1();
                bv.push_1();
                bv.push_1();
                bv.push_1();
                assert_eq!(bv.is_set(0), true);
                assert_eq!(bv.is_set(1), true);
                assert_eq!(bv.is_set(2), false);
                assert_eq!(bv.is_set(3), true);
                assert_eq!(bv.is_set(4), false);
                assert_eq!(bv.is_set(5), false);
                assert_eq!(bv.is_set(6), true);
                assert_eq!(bv.is_set(7), false);
                assert_eq!(bv.is_set(8), false);
                assert_eq!(bv.is_set(9), false);
                bv.set(2);
                bv.set(4);
                bv.set(5);
                bv.set(7);
                bv.set(8);
                assert_eq!(bv.is_set(2), true);
                assert_eq!(bv.is_set(4), true);
                assert_eq!(bv.is_set(5), true);
                assert_eq!(bv.is_set(7), true);
                assert_eq!(bv.is_set(8), true);
                for i in 0..17 {
                        bv.clear(i);
                }
                bv.set(0);
                bv.set(9);
                bv.set(16);
                bv.set(17);
                assert_eq!(bv.is_set(0), true);
                assert_eq!(bv.is_set(9), true);
                assert_eq!(bv.is_set(16), true);
                assert_eq!(bv.is_set(17), true);
        }
}