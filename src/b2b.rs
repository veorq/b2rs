const SIG: [[usize; 16]; 12] = [
    [ 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15],
    [14, 10,  4,  8,  9, 15, 13,  6,  1, 12,  0,  2, 11,  7,  5,  3],
    [11,  8, 12,  0,  5,  2, 15, 13, 10, 14,  3,  6,  7,  1,  9,  4],
    [ 7,  9,  3,  1, 13, 12, 11, 14,  2,  6,  5, 10,  4,  0, 15,  8],
    [ 9,  0,  5,  7,  2,  4, 10, 15, 14,  1, 11, 12,  6,  8,  3, 13],
    [ 2, 12,  6, 10,  0, 11,  8,  3,  4, 13,  7,  5, 15, 14,  1,  9],
    [12,  5,  1, 15, 14, 13,  4, 10,  0,  7,  6,  3,  9,  2,  8, 11],
    [13, 11,  7, 14, 12,  1,  3,  9,  5,  0, 15,  4,  8,  6,  2, 10],
    [ 6, 15, 14,  9, 11,  3,  0,  8, 12,  2, 13,  7,  1,  4, 10,  5],
    [10,  2,  8,  4,  7,  6,  1,  5, 15, 11,  9, 14,  3, 12, 13,  0],
    [ 0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15],
    [14, 10,  4,  8,  9, 15, 13,  6,  1, 12,  0,  2, 11,  7,  5,  3],
];

fn b2b_compress(h: &mut [u64; 8], t: u64, f: u64, block: &[u8] ) {

    let mut v = [
        h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7],
        0x6a09e667f3bcc908,   0xbb67ae8584caa73b, 0x3c6ef372fe94f82b,   0xa54ff53a5f1d36f1,
        0x510e527fade682d1^t, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b^f, 0x5be0cd19137e2179,
    ];

    let mut m = [0u64; 16];
    for i in 0..16 {
        for j in 0..8 {
            m[i] |= (block[i*8 + j] as u64) << (8*j);
        }
    }

macro_rules!G(
	($a: expr, $b: expr, $c: expr, $d: expr, $x: expr, $y: expr) =>
({
	v[$a] = v[$a].wrapping_add(v[$b]).wrapping_add($x);
	v[$d] = (v[$d] ^ v[$a]).rotate_right(32);
	v[$c] = v[$c].wrapping_add(v[$d]);
	v[$b] = (v[$b] ^ v[$c]).rotate_right(24);
	v[$a] = v[$a].wrapping_add(v[$b]).wrapping_add($y);
	v[$d] = (v[$d] ^ v[$a]).rotate_right(16);
	v[$c] = v[$c].wrapping_add(v[$d]);
	v[$b] = (v[$b] ^ v[$c]).rotate_right(63);
})
);

    for i in 0..12 {
        G!( 0, 4,  8, 12, m[SIG[i][ 0]], m[SIG[i][ 1]]);
        G!( 1, 5,  9, 13, m[SIG[i][ 2]], m[SIG[i][ 3]]);
        G!( 2, 6, 10, 14, m[SIG[i][ 4]], m[SIG[i][ 5]]);
        G!( 3, 7, 11, 15, m[SIG[i][ 6]], m[SIG[i][ 7]]);
        G!( 0, 5, 10, 15, m[SIG[i][ 8]], m[SIG[i][ 9]]);
        G!( 1, 6, 11, 12, m[SIG[i][10]], m[SIG[i][11]]);
        G!( 2, 7,  8, 13, m[SIG[i][12]], m[SIG[i][13]]);
        G!( 3, 4,  9, 14, m[SIG[i][14]], m[SIG[i][15]]);
    }

    for i in 0..8 { h[i] ^= v[i] ^ v[i+8]; }
}

pub fn hash(message: &[u8]) -> Vec<u8> {
    hash_custom(message, 64, 1, 1, 0, 0, 0, 0, 0)
}

pub fn hash_custom(message: &[u8], digest_length: u8, fanout: u8, depth: u8, 
    leaf_length: u32, node_offset: u32, xof_length: u32, 
    node_depth: u8, inner_length: u8
    ) -> Vec<u8> {

    let mut hash = [ 
         0x6a09e667f3bcc908 ^ (digest_length as u64) ^ (fanout as u64) << 16 
                            ^ (depth as u64) << 24 ^ (leaf_length as u64) << 32, 
         0xbb67ae8584caa73b ^ node_offset as u64 ^ (xof_length as u64) << 32, 
         0x3c6ef372fe94f82b ^ (node_depth as u64) ^ (inner_length as u64) << 8, 
         0xa54ff53a5f1d36f1,
         0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];

    let mut t = 0u64;
    let mut data = message;

    while data.len() > 128 {
        t += 128;
        let block = &data[..128];
        data = &data[128..];
        b2b_compress(&mut hash, t, 0, block);
    }

    let mut block = [0u8; 128];
    for i in 0..data.len() { block[i] = data[i]; }
    b2b_compress(&mut hash, message.len() as u64, !0, &block);

    let mut digest = Vec::new();
    for i in 0..(digest_length as usize) { digest.push((hash[i/8] >> (8*(i%8))) as u8); }
    digest
}
