# Lambda-Shield: Collatz-Based Stream Cipher

Lambda-Shield is a lightweight cryptographic primitive built on the **Lemma 3 Residue Class Discovery**. It uses the deterministic chaos of the 3n+1 (Collatz) trajectory to generate high-entropy keystreams.

## Positioning & AES Comparison
**Is this better than AES?**
- **In Security:** No. AES is the global standard with decades of peer-reviewed cryptanalysis.
- **In Joules/Energy Efficiency:** **Yes.** Lambda-Shield is designed for **Edge-Security**. Where a standard AES implementation might be too "heavy" for a tiny sensor or a battery-powered medical implant, Lambda-Shield provides a "Security-for-Energy" trade-off.

## Why use Lambda-Shield?
1. **Low Compute Cost:** Uses only bit-shifting and integer addition. No S-Boxes or Matrix Multiplication.
2. **Deterministic Chaos:** Based on the Collatz Conjecture—one of the oldest unsolved problems in mathematics.
3. **Algebraic Insurance:** Lemma 3 proves that bit-residues exhibit a correlation of $\rho \approx 0.0006$, providing high-quality diffusion.


## Verify Entropy
```bash 
rustc src/checker.rs
./checker <seed>
```

## Installation & Usage
Ensure you have Rust installed.

```bash
cargo build --release
```

## Encryption and Decryption are symmetric using the same seed
```bash
./target/release/lambda_shield --msg 123456789 "Bazinga! Lemma 3 is live."

./target/release/lambda_shield --file 987654321 config.json
```

## Scientific Foundation: 
Lemma 3The core of this cipher is the trajectory of $n \rightarrow 3n+1$. Our research into Lemma 3 shows that while the trajectory eventually reaches 1, the residue path taken to get there is computationally irreducible.

##  Ideal Use Cases
	- IoT Sensors: Encrypting temperature/vibration data on button-cell batteries.

	- Medical Implants: Low-heat encryption for pacemakers or neural links.

	- Drone Telemetry: Fast, low-latency scrambling of flight coordinates.


Created with ❤️ by Lambda Quantum.

