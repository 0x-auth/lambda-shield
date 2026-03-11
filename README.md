# 🛡️ Lambda-Shield v2: The Collatz Stream Cipher

Lambda-Shield is a high-speed, lightweight stream cipher leveraging the deterministic chaos of the **Collatz (3n+1) Trajectory**. 

## 🥊 The Competitor: ChaCha20
While **ChaCha20** is the industry standard for software-defined encryption, it requires maintaining a 512-bit state (16 x 32-bit words) and performing multiple "Quarter Round" operations involving constant arrays.

**Lambda-Shield v2 Advantages for IoT:**
- **Zero Constants:** No "expand 32-byte k" strings or magic tables.
- **Minimal State:** Operates on a single `u128` state.
- **Energy Efficiency:** Purely Arithmetic (Add, Shift, XOR, Multi). Perfect for saving Joules on button-cell powered sensors.



## 🛠️ v2 Specifications
- **Keyspace:** 128-bit (seed_hi + seed_lo).
- **Mixing:** 32 Collatz steps per byte + Murmur3-style output conditioning.
- **Symmetry:** XOR-based stream cipher (Encryption == Decryption).

## 🚀 Usage
```bash
cargo build --release
./target/release/lambda-shield --msg 12345 67890 "Your Secret Message"

```

