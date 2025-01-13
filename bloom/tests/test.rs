use bloom::{BloomFilter};


#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_push() {
                for i in 1..=4 {
                        for k in 0..=10 {
                                let mut bf = BloomFilter::new(usize::pow(100, i), k);
                                let strs = vec!["a", "e", "i", "o", "u", "y", "x", "y", "w", "ab", "ij", "az", "qp", "nm", "do", "try", "now", "line", "test", "word", "fuji", "tucson", "yukon", "saskatchewan", "simplicalcomplex", "dunfermlineraces", "Better three hours too soon than a minute too late.", "Suit the action to the word, the word to the action."];
                                for s in &strs {
                                        assert!(!bf.contains(s));
                                }
                                for s in &strs {
                                        bf.add(s);
                                        assert!(bf.contains(s));
                                }
                        }
                }
        }
}