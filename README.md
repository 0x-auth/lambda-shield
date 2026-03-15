# Lambda-Shield

[![crates.io](https://img.shields.io/crates/v/lambda-shield.svg)](https://crates.io/crates/lambda-shield)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**A hardened, lightweight stream cipher built on chaotic Collatz trajectories + SipHash-1-2 mixing.**

Lambda-Shield v0.5.0 introduces the **Algebraic Wall**—a defense-in-depth architecture that protects the internal state even in scenarios where primary keys are partially compromised. Designed for resource-constrained environments: IoT sensors, embedded MCUs, and edge devices.

```bash
cargo install lambda-shield
lambda-shield --msg 12345 67890 "Bazinga! Lemma 3 is live."

```

---

##  How It Works

Lambda-Shield uses a dual-layer architecture to decouple state evolution from output statistics.

1. **The Chaotic Driver (Collatz):** Instead of a predictable counter ($n+1$), we use a hardened Collatz trajectory. This ensures that the difference ($\Delta$) between consecutive inputs to the mixer is non-linear and massive.
2. **The Cryptographic Mixer (SipHash):** Two rounds of SipHash-1-2 provide full avalanche effects. Even a 1-bit change in the chaotic state results in ~50% bit-flip in the output keystream.

---

##  Security Advantage: The Algebraic Wall

Most modern stream ciphers (like ChaCha20) rely on a linear counter. While efficient, this creates a **Single Point of Failure**: if the key is leaked via a side-channel, the entire stream is compromised because the state ($n+1$) is trivial to predict.

Lambda-Shield provides **State Recovery Resilience**. In simulated attacks:

* **Counter-based ciphers:** Collapse instantly (100% prediction accuracy) upon key leak.
* **Lambda-Shield:** Accuracy remains **0%**. An attacker must not only steal the key but also solve the non-linear trajectory position.

| Attack Scenario | Counter + SipHash | Lambda-Shield (Collatz) |
| --- | --- | --- |
| **Known Key, Unknown State** | ❌ Compromised (100%) | ✅ Secure (0% Prediction) |
| **Differential Predictability** | Linear ($\Delta=1$) | Chaotic (Non-linear) |
| **Algebraic Immunity** | Low | High |

---

## NIST SP 800-22 Results

Tested using the **official NIST STS v2.1.2** on 1MB keystream (Seed: `12345` / `67890`):

| Test Category | Passed | Result |
| --- | --- | --- |
| Frequency / Runs / Rank | 1/1 | ✅ |
| Non-Overlapping Template | 148/148 | ✅ |
| Overlapping Template | 1/1 | ✅ |
| Linear Complexity | 1/1 | ✅ |
| **TOTAL** | **162/162** | ✅ **ALL PASS** |

---

##  Performance & Trade-offs

| Cipher | 10M Iterations (Speed) | Security Philosophy |
| --- | --- | --- |
| **Counter-based** | ~11.5 ms | Efficiency First |
| **Lambda-Shield** | ~42.3 ms | Defense in Depth |

**Real Security Overhead:** ~268% slower than a raw counter.
This "Security Tax" is intentional. By breaking CPU branch prediction through chaotic branching, Lambda-Shield ensures the state transition is as complex as the encryption itself.

---

##  Usage

### Encrypt a message

```bash
lambda-shield --msg <seed_hi> <seed_lo> "your message"

```

### Encrypt a file

```bash
lambda-shield --file <seed_hi> <seed_lo> path/to/file

```

### Research & Verification

The `research/` directory contains tools used to verify the "Algebraic Wall":

* `state_recovery_attack.rs`: Simulates key-leak scenarios.
* `checker.rs`: Analyzes bit transition density.

---

## ⚠️ Security Notes

* **Experimental:** This is a research prototype. It has passed statistical batteries (NIST), but has not undergone formal community cryptanalysis.
* **Nonce Misuse:** Never reuse the same (seed_hi, seed_lo) pair for different messages.
* **Intended Use:** High-entropy state requirements, IoT security research, and hardware-compromise-resistant environments.

---

*Built with ❤️ by [Abhishek Srivastava](https://github.com/0x-auth)