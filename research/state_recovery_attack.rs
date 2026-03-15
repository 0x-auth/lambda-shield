// state_recovery_attack.rs
//
// Simulates a partial state recovery attack on both constructions.
// Scenario: attacker has recovered 128 bits of SipHash internal state
//           (i.e. they know key0 XOR v0 and key1 XOR v1 at time T)
// Question: how many future bytes can they predict?
//
// Build: rustc -O state_recovery_attack.rs -o state_recovery_attack
// Run:   ./state_recovery_attack

use std::time::Instant;

// ── SipHash-1-2 core ─────────────────────────────────────────────────────────
fn sip_round(v0: &mut u64, v1: &mut u64, v2: &mut u64, v3: &mut u64) {
    *v0 = v0.wrapping_add(*v1); *v1 = v1.rotate_left(13); *v1 ^= *v0;
    *v0 = v0.rotate_left(32);
    *v2 = v2.wrapping_add(*v3); *v3 = v3.rotate_left(16); *v3 ^= *v2;
    *v0 = v0.wrapping_add(*v3); *v3 = v3.rotate_left(21); *v3 ^= *v0;
    *v2 = v2.wrapping_add(*v1); *v1 = v1.rotate_left(17); *v1 ^= *v2;
    *v2 = v2.rotate_left(32);
}

fn siphash_mix(state: u64, key0: u64, key1: u64) -> u64 {
    let mut v0 = key0 ^ 0x736f6d6570736575u64;
    let mut v1 = key1 ^ 0x646f72616e646f6du64;
    let mut v2 = key0 ^ 0x6c7967656e657261u64;
    let mut v3 = key1 ^ 0x7465646279746573u64;
    v3 ^= state;
    sip_round(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= state;
    v2 ^= 0xff;
    sip_round(&mut v0, &mut v1, &mut v2, &mut v3);
    sip_round(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^ v1 ^ v2 ^ v3
}

// ── Collatz step ──────────────────────────────────────────────────────────────
fn collatz_step(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else { 3u64.wrapping_mul(n).wrapping_add(1) }
}

fn collatz_next(state: u64, key0: u64, key1: u64) -> u64 {
    let mut s = collatz_step(state);
    if s == 1 || s == 2 || s == 4 {
        s = key0.wrapping_add(key1).wrapping_add(s) | 1;
    }
    s
}

// ── Generators ────────────────────────────────────────────────────────────────
struct CounterGen { counter: u64, key0: u64, key1: u64 }
impl CounterGen {
    fn new(seed: u64) -> Self {
        Self { counter: 0,
               key0: seed ^ 0xdeadbeefcafe0000,
               key1: seed.rotate_left(32) ^ 0x0123456789abcdef }
    }
    fn next(&mut self) -> u64 {
        let out = siphash_mix(self.counter, self.key0, self.key1);
        self.counter += 1;
        out
    }
}

struct CollatzGen { state: u64, key0: u64, key1: u64 }
impl CollatzGen {
    fn new(seed: u64) -> Self {
        let s = if seed % 2 == 0 { seed | 1 } else { seed };
        Self { state: s,
               key0: seed ^ 0xdeadbeefcafe0000,
               key1: seed.rotate_left(32) ^ 0x0123456789abcdef }
    }
    fn next(&mut self) -> u64 {
        self.state = collatz_next(self.state, self.key0, self.key1);
        siphash_mix(self.state, self.key0, self.key1)
    }
}

// ── Attack simulation ─────────────────────────────────────────────────────────
//
// Attack model (Kerckhoffs-compliant):
// - Attacker knows the algorithm completely
// - Attacker does NOT know the key
// - Attacker observes N output u64s (known plaintext)
// - Attacker recovers SipHash internal state at time T via side-channel
//   (simulated: we give them key0, key1 — best case for attacker)
// - Question: can they predict future outputs?
//
// For Counter: knowing key0, key1, and T means they know counter = T
//   => they can predict ALL future outputs immediately
//
// For Collatz: knowing key0, key1, and output at T does NOT reveal state at T
//   => they must also recover the Collatz trajectory position
//   => this requires inverting the Collatz map (non-trivial, branching)

fn attack_counter(seed: u64, observe_n: usize, predict_n: usize) -> f64 {
    // Legitimate stream
    let mut legit = CounterGen::new(seed);
    let mut observed: Vec<u64> = (0..observe_n).map(|_| legit.next()).collect();
    let future: Vec<u64> = (0..predict_n).map(|_| legit.next()).collect();

    // Attacker: knows key0, key1 (side-channel), knows counter = observe_n
    // They reconstruct from counter = observe_n
    let mut attacker = CounterGen::new(seed);
    attacker.counter = observe_n as u64; // <-- attacker knows this trivially

    let predicted: Vec<u64> = (0..predict_n).map(|_| attacker.next()).collect();

    let correct = future.iter().zip(predicted.iter())
        .filter(|(a, b)| a == b).count();
    
    let _ = observed.pop(); // suppress unused warning
    (correct as f64 / predict_n as f64) * 100.0
}

fn attack_collatz_with_state(seed: u64, observe_n: usize, predict_n: usize) -> f64 {
    // Legitimate stream
    let mut legit = CollatzGen::new(seed);
    let mut observed: Vec<u64> = (0..observe_n).map(|_| legit.next()).collect();
    let true_state_at_t = legit.state; // <-- attacker does NOT have this
    let future: Vec<u64> = (0..predict_n).map(|_| legit.next()).collect();

    // Attacker scenario 1: has key0, key1 but NOT Collatz state
    // They must brute-force the Collatz trajectory position
    // Collatz state space: effectively unbounded (escaped cycles go to large u64)
    // This is computationally infeasible for real keys
    // We simulate: attacker tries 10,000 candidate states (generous)
    let key0 = seed ^ 0xdeadbeefcafe0000u64;
    let key1 = seed.rotate_left(32) ^ 0x0123456789abcdef;
    let first_future = future[0];
    
    let mut found_state: Option<u64> = None;
    // Attacker tries states around where they think the trajectory might be
    // (in reality they have no idea — we give them a huge hint: search near true state)
    let search_radius = 500_000u64;
    let search_start = true_state_at_t.saturating_sub(search_radius);
    
    for candidate in search_start..=true_state_at_t.saturating_add(search_radius) {
        let candidate_state = collatz_next(candidate | 1, key0, key1);
        if siphash_mix(candidate_state, key0, key1) == first_future {
            found_state = Some(candidate | 1);
            break;
        }
    }

    let _ = observed.pop();

    match found_state {
        None => 0.0, // attacker failed to find state
        Some(s) => {
            // Attacker found a matching state — predict forward
            let mut attacker_state = collatz_next(s, key0, key1);
            let mut correct = 1usize; // first one matched by definition
            for &f in &future[1..] {
                attacker_state = collatz_next(attacker_state, key0, key1);
                if siphash_mix(attacker_state, key0, key1) == f { correct += 1; }
            }
            (correct as f64 / predict_n as f64) * 100.0
        }
    }
}

fn attack_collatz_with_full_state(seed: u64, observe_n: usize, predict_n: usize) -> f64 {
    // Best case for attacker: they somehow have BOTH key AND Collatz state
    // (complete state recovery — maximum attacker capability)
    let mut legit = CollatzGen::new(seed);
    let _observed: Vec<u64> = (0..observe_n).map(|_| legit.next()).collect();
    let true_state = legit.state;
    let future: Vec<u64> = (0..predict_n).map(|_| legit.next()).collect();

    // Attacker reconstructs perfectly
    let key0 = seed ^ 0xdeadbeefcafe0000u64;
    let key1 = seed.rotate_left(32) ^ 0x0123456789abcdef;
    let mut attacker_state = true_state;
    let predicted: Vec<u64> = (0..predict_n).map(|_| {
        attacker_state = collatz_next(attacker_state, key0, key1);
        siphash_mix(attacker_state, key0, key1)
    }).collect();

    let correct = future.iter().zip(predicted.iter())
        .filter(|(a, b)| a == b).count();
    (correct as f64 / predict_n as f64) * 100.0
}

fn main() {
    let seed = 0xab515515u64.wrapping_mul(0xdeadbeef);
    let predict_n = 128; // predict 128 * 8 = 1024 bytes

    println!("═══════════════════════════════════════════════════════════");
    println!("  STATE RECOVERY ATTACK SIMULATION");
    println!("  Lambda-Shield vs Counter+SipHash");
    println!("  Seed: 0x{:016x}", seed);
    println!("  Predict: {} u64s = {} bytes", predict_n, predict_n * 8);
    println!("═══════════════════════════════════════════════════════════\n");

    println!("SCENARIO 1: Attacker recovers SipHash key (key0, key1)");
    println!("           via side-channel at observation time T");
    println!("─────────────────────────────────────────────────────────");
    println!("{:<12} {:>20} {:>20}", "Observed", "Counter Accuracy", "Collatz Accuracy");
    println!("{:<12} {:>20} {:>20}", "  (bytes)", "(key known)", "(key only, no state)");
    println!("─────────────────────────────────────────────────────────");

    for &observe_n in &[1usize, 10, 100, 1000, 10000] {
        let t = Instant::now();
        let counter_acc = attack_counter(seed, observe_n, predict_n);
        let collatz_acc = attack_collatz_with_state(seed, observe_n, predict_n);
        println!("{:<12} {:>19.1}% {:>19.1}%   ({:.1}ms)",
            observe_n * 8, counter_acc, collatz_acc, t.elapsed().as_millis());
    }

    println!("\nSCENARIO 2: Attacker recovers COMPLETE state");
    println!("           (key0 + key1 + Collatz position)");
    println!("─────────────────────────────────────────────────────────");
    println!("{:<12} {:>20} {:>20}", "Observed", "Counter Accuracy", "Collatz Accuracy");
    println!("─────────────────────────────────────────────────────────");
    for &observe_n in &[1usize, 100, 10000] {
        let counter_acc = attack_counter(seed, observe_n, predict_n);
        let collatz_acc = attack_collatz_with_full_state(seed, observe_n, predict_n);
        println!("{:<12} {:>19.1}% {:>19.1}%",
            observe_n * 8, counter_acc, collatz_acc);
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("INTERPRETATION:");
    println!("  Scenario 1 Row: If Counter = 100%, Collatz < 100%");
    println!("    => Collatz provides meaningful second-layer protection");
    println!("  Scenario 2 Row: Both should be 100%");
    println!("    => Complete state recovery breaks both (expected)");
    println!("  The gap between Scenario 1 and Scenario 2 IS the");
    println!("    algebraic immunity Collatz provides.");
    println!("═══════════════════════════════════════════════════════════");
}
