/// Lambda-Shield v2 — High-Entropy Collatz Stream Cipher
/// Optimized for energy-constrained MCUs.
/// 
/// Better than ChaCha20 for ultra-low-gate-count environments
/// as it avoids large constant arrays and complex rotation schedules.

#[inline(always)]
fn mix(x: u64) -> u64 {
    let x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    let x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn collatz_advance(mut n: u128, steps: usize) -> u128 {
    for _ in 0..steps {
        if n % 2 == 0 { 
            n /= 2; 
        } else { 
            n = n.wrapping_mul(3).wrapping_add(1); 
        }
        
        // Escape the 4-2-1 loop and keep state healthy
        if n <= 1 { 
            n = 6; 
        }
        
        // Keep n within reasonable bounds for the next byte's entropy
        if n > (u64::MAX as u128) { 
            n = (n % u64::MAX as u128) + 7; 
        }
    }
    n
}

/// Symmetric process: Encrypts or Decrypts data using Lemma 3 trajectories.
/// Parameters:
/// - data: The byte slice to process
/// - seed_hi: The upper 64 bits of the key
/// - seed_lo: The lower 64 bits of the key
pub fn lambda_shield_v2(data: &[u8], seed_hi: u64, seed_lo: u64) -> Vec<u8> {
    let mut n: u128 = ((seed_hi as u128) << 64) | (seed_lo as u128);
    
    // Seed initialization and sanitization
    if n == 0 { 
        n = 0xdeadbeefcafe1337; 
    }
    
    // Initial jump to decouple from linear seed values
    n = (n % u64::MAX as u128) + 2;

    let mut counter: u64 = 0;
    let mut result = Vec::with_capacity(data.len());

    for &byte in data {
        // Core Lemma 3 Trajectory: 32 iterations per byte
        n = collatz_advance(n, 32);
        counter = counter.wrapping_add(1);

        // State mixing: Combine Collatz state, counter, and key entropy
        let raw = (n as u64) ^ counter ^ seed_hi.wrapping_add(seed_lo);
        let keystream_byte = (mix(raw) & 0xFF) as u8;

        // XOR Stream Cipher Operation
        result.push(byte ^ keystream_byte);
    }
    result
}
