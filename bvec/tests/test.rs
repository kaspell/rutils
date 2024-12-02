use bvec::*;


#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_push() {
                let mut bv = BitVec::new();
                let a = 27;
                for _ in 0 .. a {
                        bv.push_1();
                }
                for i in a .. 2*a {
                        if i % 2 == 0 {
                                bv.push_1();
                        } else {
                                bv.push_0();
                        }
                }
                for _ in 2*a .. 3*a {
                        bv.push_0()
                }

                for i in 0 .. a {
                        assert_eq!(bv.is_set(i), true);
                }
                for i in a .. 2*a {
                        if i % 2 == 0 {
                                assert_eq!(bv.is_set(i), true);
                        } else {
                                assert_eq!(bv.is_set(i), false);
                        }
                }
                for i in 2*a .. 3*a {
                        assert_eq!(bv.is_set(i), false);
                }
        }

        #[test]
        fn test_set_and_clear() {
                let mut bv = BitVec::new();
                let a = 51;
                for _ in 0 .. a {
                        bv.push_1();
                }
                for _ in a .. 2*a {
                        bv.push_0();
                }

                for i in 0 .. a {
                        assert_eq!(bv.is_set(i), true);
                }
                for i in a .. 2*a {
                        assert_eq!(bv.is_set(i), false);
                }

                for i in 0 .. a {
                        if i % 3 == 0 {
                                bv.clear(i);
                        }
                }
                for i in a .. 2*a {
                        if i % 4 == 0 {
                                bv.set(i);
                        }
                }

                for i in 0 .. a {
                        if i % 3 == 0 {
                                assert_eq!(bv.is_set(i), false);
                        } else {
                                assert_eq!(bv.is_set(i), true);
                        }
                }
                for i in a .. 2*a {
                        if i % 4 == 0 {
                                assert_eq!(bv.is_set(i), true);
                        } else {
                                assert_eq!(bv.is_set(i), false);
                        }
                }
        }
}