/// Struct storing the necessary state for the Message Digest 4 (MD4) hash function
/// Code is ported and `rustified` from libtomcrypt
#[allow(non_camel_case_types)]
pub struct MD4 {
    state   : [uint, ..4],
    buffer  : [u8, ..64],
    len     : u64,
    cur_len : uint
}

/// Helper function as described in the RFC
#[inline(always)]
fn f(x: uint, y: uint, z: uint) -> uint {
    z ^ (x & (y ^ z))
}

/// Helper function as described in the RFC
#[inline(always)]
fn g(x: uint, y: uint, z: uint) -> uint {
    (x & y) | (z & (x | y))
}

/// Helper function as described in the RFC
#[inline(always)]
fn h(x: uint, y: uint, z: uint) -> uint {
    x ^ y ^ z
}

impl MD4 {
    /// Creates a new MD4 instance
    ///
    /// # Returns
    /// * The created instance
    pub fn new() -> MD4 {
        MD4 {
            state   : [0, ..4],
            buffer  : [0, ..64],
            len     : 0,
            cur_len : 0
        }
    }

    fn reset(&mut self) {
        self.state   = [0, ..4];
        self.buffer  = [0, ..64];
        self.len     = 0;
        self.cur_len = 0;
    }
}

impl ::hashes::HashFunction for MD4 {
    fn set_input(&mut self, input: &[u8]) {

        // First reset the hash state
        self.reset();
    }

    fn hash(&mut self) {

    }

    fn get_output(&mut self, output: &mut [u8]) {
        assert!(output.len() >= self.get_output_length())
    }

    fn get_blocksize(&self) -> uint { 64 }

    fn get_output_length_in_bits(&self) -> uint { 128 }
}

#[cfg(test)]
mod tests {
    use hashes::md4::MD4;
    use hashes::test::{HashTestCase, perform_hash_test};

    #[test]
    fn test_md4() {
        let tests = vec![
            HashTestCase {
                input: "",
                output: vec![
                    0x31,0xd6,0xcf,0xe0,0xd1,0x6a,0xe9,0x31,
                    0xb7,0x3c,0x59,0xd7,0xe0,0xc0,0x89,0xc0
                ],
                output_str: "31d6cfe0d16ae931b73c59d7e0c089c0"
            },
            HashTestCase {
                input: "a",
                output: vec![
                    0xbd,0xe5,0x2c,0xb3,0x1d,0xe3,0x3e,0x46,
                    0x24,0x5e,0x05,0xfb,0xdb,0xd6,0xfb,0x24
                ],
                output_str: "bde52cb31de33e46245e05fbdbd6fb24"
            },
            HashTestCase {
                input: "abc",
                output: vec![
                    0xa4,0x48,0x01,0x7a,0xaf,0x21,0xd8,0x52,
                    0x5f,0xc1,0x0a,0xe8,0x7a,0xa6,0x72,0x9d
                ],
                output_str: "a448017aaf21d8525fc10ae87aa6729d"
            },
            HashTestCase {
                input: "message digest",
                output: vec![
                    0xd9,0x13,0x0a,0x81,0x64,0x54,0x9f,0xe8,
                    0x18,0x87,0x48,0x06,0xe1,0xc7,0x01,0x4b
                ],
                output_str: "d9130a8164549fe818874806e1c7014b"
            },
            HashTestCase {
                input: "abcdefghijklmnopqrstuvwxyz",
                output: vec![
                    0xd7,0x9e,0x1c,0x30,0x8a,0xa5,0xbb,0xcd,
                    0xee,0xa8,0xed,0x63,0xdf,0x41,0x2d,0xa9
                ],
                output_str: "d79e1c308aa5bbcdeea8ed63df412da9"
            },
            HashTestCase {
                input: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                output: vec![
                    0x04,0x3f,0x85,0x82,0xf2,0x41,0xdb,0x35,
                    0x1c,0xe6,0x27,0xe1,0x53,0xe7,0xf0,0xe4
                ],
                output_str: "043f8582f241db351ce627e153e7f0e4"
            },
            HashTestCase {
                input: "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
                output: vec![
                    0xe3,0x3b,0x4d,0xdc,0x9c,0x38,0xf2,0x19,
                    0x9c,0x3e,0x7b,0x16,0x4f,0xcc,0x05,0x36
                ],
                output_str: "e33b4ddc9c38f2199c3e7b164fcc0536"
            }
        ];

        let mut md4 = MD4::new();
        for t in tests.iter() {
            perform_hash_test(&mut md4, t);
        }
    }
}
