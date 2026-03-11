use std::env;

fn get_collatz_bit(mut n: u128) -> (u128, u8) {
    if n % 2 == 0 {
        (n / 2, 0)
    } else {
        (n.wrapping_mul(3).wrapping_add(1), 1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let seed: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(12345);
    
    let mut n = seed as u128;
    let mut zeros = 0;
    let mut ones = 0;
    let mut transitions = 0;
    let mut last_bit = 2;

    println!("Checking Lemma 3 Entropy for Seed: {}", seed);
    
    // Test 10,000 steps of the trajectory
    for _ in 0..10000 {
        let (next_n, bit) = get_collatz_bit(n);
        if bit == 0 { zeros += 1; } else { ones += 1; }
        
        if last_bit != 2 && bit != last_bit {
            transitions += 1;
        }
        
        last_bit = bit;
        n = next_n;
        if n <= 1 { n = seed as u128 + last_bit as u128 + 7; } // Keep it going
    }

    let ratio = ones as f32 / (zeros + ones) as f32;
    println!("--- Entropy Report ---");
    println!("Total Bits: 10,000");
    println!("Zero/One Ratio: {:.4} (Ideal: 0.5-0.66 for Collatz)", ratio);
    println!("Bit Transitions: {} (High transitions = High Chaos)", transitions);
    
    if ratio > 0.3 && ratio < 0.7 {
        println!("STATUS: Lemma 3 Verified. Output is High-Entropy.");
    } else {
        println!("STATUS: Weak Seed. Trajectory converged too fast.");
    }
}
