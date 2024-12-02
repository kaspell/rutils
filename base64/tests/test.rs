use base64::*;


#[cfg(test)]
mod tests {
        use super::*;

        // Test that all bytes can be encoded and decoded
        #[test]
        fn test_each_byte() {
                let bytes = (0 ..= 255).collect::<Vec::<u8>>();
                let encoded = b64enc(bytes.as_slice());
                let decoded = b64dec(&encoded);
                assert_eq!(encoded, "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn+AgYKDhIWGh4iJiouMjY6PkJGSk5SVlpeYmZqbnJ2en6ChoqOkpaanqKmqq6ytrq+wsbKztLW2t7i5uru8vb6/wMHCw8TFxsfIycrLzM3Oz9DR0tPU1dbX2Nna29zd3t/g4eLj5OXm5+jp6uvs7e7v8PHy8/T19vf4+fr7/P3+/w==".to_string());
                assert_eq!(decoded, bytes);
        }

        #[test]
        fn test_padding() {
                let a = b64enc(b"");
                let b = b64enc(b"a");
                let c = b64enc(b"%g");
                let d = b64enc(b"gcc");
                assert_eq!(a, "".to_string());
                assert_eq!(b, "YQ==".to_string());
                assert_eq!(c, "JWc=".to_string());
                assert_eq!(d, "Z2Nj".to_string());
                assert_eq!(b64dec(&a), b"");
                assert_eq!(b64dec(&b), b"a");
                assert_eq!(b64dec(&c), b"%g");
                assert_eq!(b64dec(&d), b"gcc");

        }

        #[test]
        fn test_ascii() {
                // The collection of printable ASCII characters
                let ascii_symbols: &[u8; 95] = b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghjiklmnopqrstuvwxyz{|}~";
                let encoded = b64enc(ascii_symbols);
                let decoded = b64dec(&encoded);
                assert_eq!(encoded, "ICEiIyQlJicoKSorLC0uLzAxMjM0NTY3ODk6Ozw9Pj9AQUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVpbXF1eX2BhYmNkZWZnaGppa2xtbm9wcXJzdHV2d3h5ent8fX4=".to_string());
                assert_eq!(decoded, ascii_symbols);
        }

        #[test]
        fn test_shakespeare() {
                let scene = b"An earnest conjuration from the king,\nAs England was his faithful tributary,\nAs love between them like the palm might flourish,\nAs peace should stiff her wheaten garland wear\nAnd stand a comma 'tween their amities,\nAnd many such-like 'As'es of great charge,\nThat, on the view and knowing of these contents,\nWithout debatement further, more or less,\nHe should the bearers put to sudden death,\nNot shriving-time allow'd.";
                let encoded = b64enc(scene);
                let decoded = b64dec(&encoded);
                assert_eq!(encoded, "QW4gZWFybmVzdCBjb25qdXJhdGlvbiBmcm9tIHRoZSBraW5nLApBcyBFbmdsYW5kIHdhcyBoaXMgZmFpdGhmdWwgdHJpYnV0YXJ5LApBcyBsb3ZlIGJldHdlZW4gdGhlbSBsaWtlIHRoZSBwYWxtIG1pZ2h0IGZsb3VyaXNoLApBcyBwZWFjZSBzaG91bGQgc3RpZmYgaGVyIHdoZWF0ZW4gZ2FybGFuZCB3ZWFyCkFuZCBzdGFuZCBhIGNvbW1hICd0d2VlbiB0aGVpciBhbWl0aWVzLApBbmQgbWFueSBzdWNoLWxpa2UgJ0FzJ2VzIG9mIGdyZWF0IGNoYXJnZSwKVGhhdCwgb24gdGhlIHZpZXcgYW5kIGtub3dpbmcgb2YgdGhlc2UgY29udGVudHMsCldpdGhvdXQgZGViYXRlbWVudCBmdXJ0aGVyLCBtb3JlIG9yIGxlc3MsCkhlIHNob3VsZCB0aGUgYmVhcmVycyBwdXQgdG8gc3VkZGVuIGRlYXRoLApOb3Qgc2hyaXZpbmctdGltZSBhbGxvdydkLg==".to_string());
                assert_eq!(decoded, scene);

        }
}