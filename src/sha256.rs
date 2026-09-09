//! Vendored FIPS 180-4 SHA-256 implementation.
//! Zero heap allocations, zero external crate dependencies.
//! Pure #![no_std] using fixed-bound loops.

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];
    let mut i = 0;
    while i < 16 {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
        i += 1;
    }
    while i < 64 {
        let x = w[i - 15];
        let s0 = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
        let y = w[i - 2];
        let s1 = y.rotate_right(17) ^ y.rotate_right(19) ^ (y >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
        i += 1;
    }
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];
    i = 0;
    while i < 64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let t1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
        i += 1;
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

pub fn sha256_32(input: &[u8]) -> [u8; 32] {
    let mut state = [
        0x6a09e667u32, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c,
        0x1f83d9ab, 0x5be0cd19,
    ];
    let mut off = 0usize;
    while off + 64 <= input.len() {
        let mut block = [0u8; 64];
        block.copy_from_slice(&input[off..off + 64]);
        compress(&mut state, &block);
        off += 64;
    }
    let mut tail = [0u8; 128];
    let rem = input.len() - off;
    tail[..rem].copy_from_slice(&input[off..]);
    tail[rem] = 0x80;
    let bit_len = (input.len() as u64) * 8;
    let pad_len = if rem + 1 + 8 <= 64 { 64 } else { 128 };
    let be = bit_len.to_be_bytes();
    tail[pad_len - 8..pad_len].copy_from_slice(&be);
    let mut block = [0u8; 64];
    block.copy_from_slice(&tail[..64]);
    compress(&mut state, &block);
    if pad_len == 128 {
        block.copy_from_slice(&tail[64..128]);
        compress(&mut state, &block);
    }
    let mut out = [0u8; 32];
    let mut j = 0;
    while j < 8 {
        out[j * 4..j * 4 + 4].copy_from_slice(&state[j].to_be_bytes());
        j += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_hex(bytes: &[u8; 32]) -> [u8; 64] {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut hex = [0u8; 64];
        for i in 0..32 {
            hex[i * 2] = HEX[(bytes[i] >> 4) as usize];
            hex[i * 2 + 1] = HEX[(bytes[i] & 0x0f) as usize];
        }
        hex
    }

    #[test]
    fn test_nist_empty_string() {
        let hash = sha256_32(b"");
        let hex = to_hex(&hash);
        assert_eq!(&hex[..], b"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_nist_abc() {
        let hash = sha256_32(b"abc");
        let hex = to_hex(&hash);
        assert_eq!(&hex[..], b"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn test_nist_two_blocks() {
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let hash = sha256_32(msg);
        let hex = to_hex(&hash);
        assert_eq!(&hex[..], b"248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
    }
}
