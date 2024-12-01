use base64::*;


#[cfg(test)]
mod tests {
        use super::*;

        // The set of printable ASCII characters
        const ASCII_CHARS: &[u8; 95] = b" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghjiklmnopqrstuvwxyz{|}~";

        #[test]
        fn test_base64_encoding() {
                assert_eq!(b64enc(b""), "".to_string());
                assert_eq!(b64enc(b"a"), "YQ==".to_string());
                assert_eq!(b64enc(b"%g"), "JWc=".to_string());
                assert_eq!(b64enc(b"gcc"), "Z2Nj".to_string());
                assert_eq!(b64enc(ASCII_CHARS), "ICEiIyQlJicoKSorLC0uLzAxMjM0NTY3ODk6Ozw9Pj9AQUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVpbXF1eX2BhYmNkZWZnaGppa2xtbm9wcXJzdHV2d3h5ent8fX4=".to_string());
                assert_eq!(b64enc(b"An earnest conjuration from the king,\nAs England was his faithful tributary,\nAs love between them like the palm might flourish,\nAs peace should stiff her wheaten garland wear\nAnd stand a comma 'tween their amities,\nAnd many such-like 'As'es of great charge,\nThat, on the view and knowing of these contents,\nWithout debatement further, more or less,\nHe should the bearers put to sudden death,\nNot shriving-time allow'd."), "QW4gZWFybmVzdCBjb25qdXJhdGlvbiBmcm9tIHRoZSBraW5nLApBcyBFbmdsYW5kIHdhcyBoaXMgZmFpdGhmdWwgdHJpYnV0YXJ5LApBcyBsb3ZlIGJldHdlZW4gdGhlbSBsaWtlIHRoZSBwYWxtIG1pZ2h0IGZsb3VyaXNoLApBcyBwZWFjZSBzaG91bGQgc3RpZmYgaGVyIHdoZWF0ZW4gZ2FybGFuZCB3ZWFyCkFuZCBzdGFuZCBhIGNvbW1hICd0d2VlbiB0aGVpciBhbWl0aWVzLApBbmQgbWFueSBzdWNoLWxpa2UgJ0FzJ2VzIG9mIGdyZWF0IGNoYXJnZSwKVGhhdCwgb24gdGhlIHZpZXcgYW5kIGtub3dpbmcgb2YgdGhlc2UgY29udGVudHMsCldpdGhvdXQgZGViYXRlbWVudCBmdXJ0aGVyLCBtb3JlIG9yIGxlc3MsCkhlIHNob3VsZCB0aGUgYmVhcmVycyBwdXQgdG8gc3VkZGVuIGRlYXRoLApOb3Qgc2hyaXZpbmctdGltZSBhbGxvdydkLg==".to_string());
        }

        #[test]
        fn test_base64_decoding() {
                assert_eq!(b64dec(&"".to_string()), b"");
                assert_eq!(b64dec(&"YQ==".to_string()), b"a");
                assert_eq!(b64dec(&"JWc=".to_string()), b"%g");
                assert_eq!(b64dec(&"Z2Nj".to_string()), b"gcc");
                assert_eq!(b64dec(&"ICEiIyQlJicoKSorLC0uLzAxMjM0NTY3ODk6Ozw9Pj9AQUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVpbXF1eX2BhYmNkZWZnaGppa2xtbm9wcXJzdHV2d3h5ent8fX4=".to_string()), ASCII_CHARS);
                assert_eq!(b64dec(&"QW4gZWFybmVzdCBjb25qdXJhdGlvbiBmcm9tIHRoZSBraW5nLApBcyBFbmdsYW5kIHdhcyBoaXMgZmFpdGhmdWwgdHJpYnV0YXJ5LApBcyBsb3ZlIGJldHdlZW4gdGhlbSBsaWtlIHRoZSBwYWxtIG1pZ2h0IGZsb3VyaXNoLApBcyBwZWFjZSBzaG91bGQgc3RpZmYgaGVyIHdoZWF0ZW4gZ2FybGFuZCB3ZWFyCkFuZCBzdGFuZCBhIGNvbW1hICd0d2VlbiB0aGVpciBhbWl0aWVzLApBbmQgbWFueSBzdWNoLWxpa2UgJ0FzJ2VzIG9mIGdyZWF0IGNoYXJnZSwKVGhhdCwgb24gdGhlIHZpZXcgYW5kIGtub3dpbmcgb2YgdGhlc2UgY29udGVudHMsCldpdGhvdXQgZGViYXRlbWVudCBmdXJ0aGVyLCBtb3JlIG9yIGxlc3MsCkhlIHNob3VsZCB0aGUgYmVhcmVycyBwdXQgdG8gc3VkZGVuIGRlYXRoLApOb3Qgc2hyaXZpbmctdGltZSBhbGxvdydkLg==".to_string()), b"An earnest conjuration from the king,\nAs England was his faithful tributary,\nAs love between them like the palm might flourish,\nAs peace should stiff her wheaten garland wear\nAnd stand a comma 'tween their amities,\nAnd many such-like 'As'es of great charge,\nThat, on the view and knowing of these contents,\nWithout debatement further, more or less,\nHe should the bearers put to sudden death,\nNot shriving-time allow'd.");
        }
}