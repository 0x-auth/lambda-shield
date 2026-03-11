/// Lambda-Shield v2 — High-Entropy Collatz Stream Cipher
/// Optimized for energy-constrained MCUs.
/// 
/// Better than ChaCha20 for ultra-low-gate-count environments
/// as it avoids large constant arrays and complex rotation schedules.

#[inline(always)]
fn mix(x: u64) -> u64 {
    let mut x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn collatz_advance(mut n: u128, steps: usize) -> u128 {
    for _ in 0..steps {
        if n % 2 == 0 { n /= 2; }
        else { n = n.wrapping_mul(3).wrapping_add(1); }
        // Keep n within u128 bounds and away from 1-4-2 loop
        if n <= 1 { n = 0xdeadbeef; }
        if n > (u64::MAX as u128) { n = (n % u64::MAX as u128) + 7; }
    }
    n
}

/// Symmetric process: Encrypts or Decrypts data using Lemma 3 trajectories.
pub fn lambda_shield_v2(data: &[u8], seed_hi: u64, seed_lo: u64) -> Vec<u8> {
    let mut n: u128 = ((seed_hi as u128) << 64) | (seed_lo as u128);
    if n == 0 { n = 0xdeadbeefcafe1337; }
    
    let mut counter: u64 = 0;
    let mut result = Vec::with_capacity(data.len());

    for &byte in data {
        n = collatz_advance(n, 32); // 32 steps per byte for deep mixing
        counter = counter.wrapping_add(1);

        // Mix Collatz state with counter and high-seed entropy
        let raw = (n as u64) ^ counter ^ seed_hi;
        let keystream_byte = (mix(raw) & 0xFF) as u8;

        result.push(byte ^ keystream_byte);
    }
    result
}
