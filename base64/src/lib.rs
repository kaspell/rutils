//! base64 encoding and decoding of byte sequences


static B64_BTREE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

// + as usize = 43
// / as usize = 47
// 0 as usize = 48
// 9 as usize = 57
// A as usize = 65
// Z as usize = 90
// a as usize = 97
// z as usize = 122
//                          +               /   0   1   2   3   4   5   6   7   8   9                               A  B  C  D  E  F  G  H  I  J  K   L   M   N   O   P   Q   R   S   T   U   V   W   X   Y   Z                           a   b   c   d   e   f   g   h   i   j   k   l   m   n   o   p   q   r   s   t   u   v   w   x   y   z
static B64_POS: [u8; 80] = [62, 64, 64, 64, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 64, 64, 64, 64, 64, 64, 64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 64, 64, 64, 64, 64, 64, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51];


pub fn b64enc(bytes: &[u8]) -> String {
        let mut ret_string: String = Default::default();
        let mut btree_idx: usize = 0;

        // Extract data from the bytes
        for byte in bytes {
                for i in (0..8).rev() {
                        btree_idx = 2*btree_idx + 1;
                        if ((1u8 << i) & *byte) > 0 { // Is the bit set?
                                btree_idx += 1
                        }
                        if btree_idx > 62 {
                                ret_string.push(B64_BTREE[btree_idx - 63]);
                                btree_idx = 0;
                        }
                }
        }

        // Add padding bits
        if btree_idx != 0 {
                let twice = btree_idx < 15;
                while btree_idx < 63 {
                        btree_idx = 2*btree_idx + 1;
                }
                ret_string.push(B64_BTREE[btree_idx - 63]);
                ret_string.push_str("=");
                if twice {
                        ret_string.push_str("=");
                }
        }
        ret_string
}

pub fn b64dec(s: &String) -> Vec<u8> {
        let mut ret_bytes = Vec::new();
        let mut byte = 0u8;
        let mut bit_idx: usize = 7;
        for c in s.chars() {
                if c == '=' {
                        break;
                }
                let bits = B64_POS[c as usize - 43];
                for i in (0..6).rev() {
                        if (1u8 << i) & bits > 0 {
                                byte |= 1u8 << bit_idx;
                        }
                        if bit_idx == 0 {
                                ret_bytes.push(byte);
                                byte = 0u8;
                                bit_idx = 8;
                        }
                        bit_idx -= 1;
                }
        }
        ret_bytes
}