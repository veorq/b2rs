use b2b;

pub fn hash(message: &[u8], hashlen: u32) -> Vec<u8> {
    let h0 = b2b::hash_custom(message, 64, 1, 1, 0, 0, hashlen, 0, 0);
    let mut outlen: u32 = hashlen;
    let mut offset: u32 = 0;
    let mut digest = Vec::new();
    while outlen != 0 {
        let digest_length: u8 = if outlen >= 64 { 64 } else { outlen as u8};
        let mut h = b2b::hash_custom(&h0[..], digest_length, 0, 0, 64, offset, hashlen, 0, 64);
        digest.append(&mut h);
        outlen -= digest_length as u32;
        offset += 1;
    }
    digest
}
