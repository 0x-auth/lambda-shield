# 🛡️ Lambda-Shield v4: Hardened Collatz Stream Cipher

Lambda-Shield v4 is a high-assurance, lightweight stream cipher. It combines the **deterministic chaos** of Collatz trajectories with the **Avalanche effect** of SipHash-1-2 rounds.

## 🚀 What's New in v4 (The Hardened Edition)
- **256-bit Internal State:** Driven by a 128-bit user seed.
- **SipHash-1-2 Rounds:** Every bit of the Collatz state is diffused through ARX (Add-Rotate-XOR) operations for maximum statistical cleanliness.
- **NIST-Ready:** Includes a `--nist` mode to generate 1MB+ keystreams for SP 800-22 statistical testing.
- **Constant-Time Logic:** Designed to be resistant to simple timing attacks on embedded hardware.



## 📊 Comparison: Lambda-Shield vs ChaCha20
| Feature | ChaCha20 | Lambda-Shield v4 |
| :--- | :--- | :--- |
| **State Size** | 512-bit | **256-bit** (Leaner) |
| **Constants** | 128-bit fixed | **None** (Fully dynamic) |
| **Math** | ARX | **Collatz + ARX** |
| **IoT Gate Count** | Medium | **Ultra-Low** |

## 🛠️ Usage
### String Encryption
```bash
# ./lambda_shield --msg <seed_hi> <seed_lo> "message"
./target/release/lambda_shield --msg 12345 67890 "Bazinga! Lemma 3 v4 is live."

```

### NIST Statistical Testing

```bash
./target/release/lambda_shield --nist 12345 67890
# Generates keystream.bin for use with NIST STS

```

## 🔬 Scientific Foundation

Lambda-Shield v4 uses the Collatz map as a **Non-Linear Feedback Shifter**. By injecting the trajectory bits into a SipHash state, we ensure that the output is indistinguishable from white noise, satisfying the "Lemma 3" residue chaos property.

---

Created with ❤️ by **Lambda Quantum**.

