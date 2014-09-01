use hashes::HashFunction;

static MD2_S_Table: &'static [u8] = &[
    0x29, 0x2E, 0x43, 0xC9, 0xA2, 0xD8, 0x7C, 0x01, 0x3D, 0x36, 0x54, 0xA1, 0xEC, 0xF0, 0x06, 0x13,
    0x62, 0xA7, 0x05, 0xF3, 0xC0, 0xC7, 0x73, 0x8C, 0x98, 0x93, 0x2B, 0xD9, 0xBC, 0x4C, 0x82, 0xCA,
    0x1E, 0x9B, 0x57, 0x3C, 0xFD, 0xD4, 0xE0, 0x16, 0x67, 0x42, 0x6F, 0x18, 0x8A, 0x17, 0xE5, 0x12,
    0xBE, 0x4E, 0xC4, 0xD6, 0xDA, 0x9E, 0xDE, 0x49, 0xA0, 0xFB, 0xF5, 0x8E, 0xBB, 0x2F, 0xEE, 0x7A,
    0xA9, 0x68, 0x79, 0x91, 0x15, 0xB2, 0x07, 0x3F, 0x94, 0xC2, 0x10, 0x89, 0x0B, 0x22, 0x5F, 0x21,
    0x80, 0x7F, 0x5D, 0x9A, 0x5A, 0x90, 0x32, 0x27, 0x35, 0x3E, 0xCC, 0xE7, 0xBF, 0xF7, 0x97, 0x03,
    0xFF, 0x19, 0x30, 0xB3, 0x48, 0xA5, 0xB5, 0xD1, 0xD7, 0x5E, 0x92, 0x2A, 0xAC, 0x56, 0xAA, 0xC6,
    0x4F, 0xB8, 0x38, 0xD2, 0x96, 0xA4, 0x7D, 0xB6, 0x76, 0xFC, 0x6B, 0xE2, 0x9C, 0x74, 0x04, 0xF1,
    0x45, 0x9D, 0x70, 0x59, 0x64, 0x71, 0x87, 0x20, 0x86, 0x5B, 0xCF, 0x65, 0xE6, 0x2D, 0xA8, 0x02,
    0x1B, 0x60, 0x25, 0xAD, 0xAE, 0xB0, 0xB9, 0xF6, 0x1C, 0x46, 0x61, 0x69, 0x34, 0x40, 0x7E, 0x0F,
    0x55, 0x47, 0xA3, 0x23, 0xDD, 0x51, 0xAF, 0x3A, 0xC3, 0x5C, 0xF9, 0xCE, 0xBA, 0xC5, 0xEA, 0x26,
    0x2C, 0x53, 0x0D, 0x6E, 0x85, 0x28, 0x84, 0x09, 0xD3, 0xDF, 0xCD, 0xF4, 0x41, 0x81, 0x4D, 0x52,
    0x6A, 0xDC, 0x37, 0xC8, 0x6C, 0xC1, 0xAB, 0xFA, 0x24, 0xE1, 0x7B, 0x08, 0x0C, 0xBD, 0xB1, 0x4A,
    0x78, 0x88, 0x95, 0x8B, 0xE3, 0x63, 0xE8, 0x6D, 0xE9, 0xCB, 0xD5, 0xFE, 0x3B, 0x00, 0x1D, 0x39,
    0xF2, 0xEF, 0xB7, 0x0E, 0x66, 0x58, 0xD0, 0xE4, 0xA6, 0x77, 0x72, 0xF8, 0xEB, 0x75, 0x4B, 0x0A,
    0x31, 0x44, 0x50, 0xB4, 0x8F, 0xED, 0x1F, 0x1A, 0xDB, 0x99, 0x8D, 0x33, 0x9F, 0x11, 0x83, 0x14
];

/// Struct storing the necessary state for the Message Digest 2 (MD2) hash function
/// Code is ported and `rustified` from libtomcrypt
#[allow(non_camel_case_types)]
pub struct MD2 {
    check_sum : [u8, ..16],
    x         : [u8, ..48],
    buffer    : [u8, ..16],
    cur_len   : uint
}

fn md2_compress(state: &mut MD2) {
   /* copy block to state.x */
   for i in range(0, 16) {
       state.x[16 + i] = state.buffer[i];
       state.x[32 + i] = state.x[i] ^ state.x[16 + i];
   }

   let mut t = 0u8;
   /* perform 18 rounds */
   for round in range(0, 18) {
       for i in range(0, 48) {
           state.x[i] ^= MD2_S_Table[(t & 255) as uint];
           t = state.x[i];
       }
       t = t + round & 255;
   }
}

#[allow(non_snake_case)]
fn md2_update_checksum(state: &mut MD2) {
    let mut L = state.check_sum[15];
    for i in range(0, 16) {
        /* caution, the RFC says its "C[j] = S[M[i*16+j] xor L]" but the reference
         * source code [and test vectors] say otherwise. */
        state.check_sum[i] ^= MD2_S_Table[(state.buffer[i] ^ L) as uint] & 255;
        L = state.check_sum[i];
    }
}

impl MD2 {
    pub fn new() -> MD2 {
        MD2 {
            check_sum : [0, ..16],
            x         : [0, ..48],
            buffer    : [0, ..16],
            cur_len   : 0
        }
    }

    fn reset(&mut self) {
        self.check_sum = [0, ..16];
        self.x         = [0, ..48];
        self.buffer    = [0, ..16];
        self.cur_len   = 0;
    }
}

impl HashFunction for MD2 {
    fn set_input(&mut self, input: &[u8]) {
        use std::cmp::{min};

        self.reset();

        // When is this the case?
        if self.cur_len > self.buffer.len() {
           fail!()
        }

        let mut index = 0u;
        let mut in_len = input.len();

        loop {
            if in_len <= 0 {
                break;
            }

            let n = min(in_len, (16 - self.cur_len));
            for i in range(0, n) {
                self.buffer[self.cur_len + i] = input[index + i];
            }
            self.cur_len += n;
            index         += n;
            in_len       -= n;

            /* if 16 bytes are filled compress and update checksum */
            if self.cur_len == 16 {
                md2_compress(self);
                md2_update_checksum(self);
                self.cur_len = 0;
            }
        }
    }

    fn hash(&mut self) {
        // Again when is this the case?
        if self.cur_len >= self.buffer.len() {
           fail!()
        }

        /* pad the message */
        let k: u8 = 16u8 - self.cur_len as u8;
        for i in range(self.cur_len, 16) {
            self.buffer[i] = k;
        }

        /* hash and update */
        md2_compress(self);
        md2_update_checksum(self);

        /* hash checksum */
        for i in range(0, 16) {
            self.buffer[i] = self.check_sum[i];
        }
        md2_compress(self);
    }

    fn get_output(&mut self, output: &mut [u8]) {
        assert!(output.len() >= self.get_output_length())

        for i in range(0, 16) {
            output[i] = self.x[i];
        }
    }

    fn get_blocksize(&self) -> uint { 16 }

    fn get_output_length_in_bits(&self) -> uint { 128 }
}

#[cfg(test)]
mod tests {
    use hashes::md2::MD2;
    use hashes::test::{HashTestCase, perform_hash_test};

    #[test]
    fn test_md2() {
        let tests = vec![
            HashTestCase {
                input: "",
                output: vec![
                    0x83,0x50,0xe5,0xa3,0xe2,0x4c,0x15,0x3d,
                    0xf2,0x27,0x5c,0x9f,0x80,0x69,0x27,0x73
                ],
                output_str: ""
            },
            HashTestCase {
                input: "a",
                output: vec![
                    0x32,0xec,0x01,0xec,0x4a,0x6d,0xac,0x72,
                    0xc0,0xab,0x96,0xfb,0x34,0xc0,0xb5,0xd1
                ],
                output_str: ""
            },
            HashTestCase {
                input: "message digest",
                output: vec![
                    0xab,0x4f,0x49,0x6b,0xfb,0x2a,0x53,0x0b,
                    0x21,0x9f,0xf3,0x30,0x31,0xfe,0x06,0xb0
                ],
                output_str: ""
            },
            HashTestCase {
                input: "abcdefghijklmnopqrstuvwxyz",
                output: vec![
                    0x4e,0x8d,0xdf,0xf3,0x65,0x02,0x92,0xab,
                    0x5a,0x41,0x08,0xc3,0xaa,0x47,0x94,0x0b
                ],
                output_str: ""
            },
            HashTestCase {
                input: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
                output: vec![
                    0xda,0x33,0xde,0xf2,0xa4,0x2d,0xf1,0x39,
                    0x75,0x35,0x28,0x46,0xc3,0x03,0x38,0xcd
                ],
                output_str: ""
            },
            HashTestCase {
                input: "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
                output: vec![
                    0xd5,0x97,0x6f,0x79,0xd8,0x3d,0x3a,0x0d,
                    0xc9,0x80,0x6c,0x3c,0x66,0xf3,0xef,0xd8
                ],
                output_str: ""
            }
        ];

        let mut md2 = MD2::new();
        for t in tests.iter() {
            perform_hash_test(&mut md2, t);
        }
    }
}
