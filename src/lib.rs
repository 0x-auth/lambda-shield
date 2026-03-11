/// Lambda-Shield v4 — Hardened Collatz Stream Cipher
/// Using SipHash-1-2 rounds for maximum avalanche effect.

#[inline(always)]
fn sip_round(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> (u64, u64, u64, u64) {
    v0 = v0.wrapping_add(v1); v2 = v2.wrapping_add(v3);
    v1 = v1.rotate_left(13);  v3 = v3.rotate_left(16);
    v1 ^= v0; v3 ^= v2;
    v0 = v0.rotate_left(32);
    v2 = v2.wrapping_add(v1); v0 = v0.wrapping_add(v3);
    v1 = v1.rotate_left(17);  v3 = v3.rotate_left(21);
    v1 ^= v2; v3 ^= v0;
    v2 = v2.rotate_left(32);
    (v0, v1, v2, v3)
}

fn collatz_step(mut n: u128) -> u128 {
    if n % 2 == 0 { n /= 2; }
    else { n = n.wrapping_mul(3).wrapping_add(1); }
    if n <= 1 { n = 0xdeadbeef1337cafe; }
    if n > (u64::MAX as u128) { n = (n % u64::MAX as u128) + 7; }
    n
}

pub fn lambda_process(data: &[u8], seed_hi: u64, seed_lo: u64) -> Vec<u8> {
    let mut n = ((seed_hi as u128) << 64) | (seed_lo as u128);
    let mut v0 = seed_hi ^ 0x736f6d6570736575;
    let mut v1 = seed_lo ^ 0x646f72616e646f6d;
    let mut v2 = seed_hi ^ 0x6c7967656e657261;
    let mut v3 = seed_lo ^ 0x7465646279746573;

    let mut result = Vec::with_capacity(data.len());
    let mut counter: u64 = 0;

    for &byte in data {
        // Advance Collatz
        for _ in 0..8 { n = collatz_step(n); }
        
        // Inject Collatz state and counter into SipHash state
        v3 ^= n as u64 ^ counter;
        let (r0, r1, r2, r3) = sip_round(v0, v1, v2, v3);
        v0 = r0; v1 = r1; v2 = r2; v3 = r3;
        
        let keystream_byte = (v0 ^ v1 ^ v2 ^ v3) as u8;
        result.push(byte ^ keystream_byte);
        counter = counter.wrapping_add(1);
    }
    result
}

pub fn lambda_keystream(len: usize, seed_hi: u64, seed_lo: u64) -> Vec<u8> {
    let dummy = vec![0u8; len];
    lambda_process(&dummy, seed_hi, seed_lo)
}
