# 🛡️ Lambda-Shield

[![crates.io](https://img.shields.io/crates/v/lambda-shield.svg)](https://crates.io/crates/lambda-shield)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**A lightweight stream cipher built on Collatz trajectories + SipHash-1-2 mixing.**

Designed for resource-constrained environments: IoT sensors, embedded MCUs, edge devices. No S-boxes, no lookup tables — just integer add, XOR, and rotate.

```bash
cargo install lambda-shield
lambda-shield --msg 12345 67890 "Bazinga! Lemma 3 is live."
```

---

## How It Works

Lambda-Shield uses two components in series:

```
128-bit seed
     │
     ▼
┌─────────────────────┐
│  Collatz Trajectory  │  ← non-algebraic state driver (16 steps/block)
│  n → n/2 or 3n+1    │
└────────┬────────────┘
         │  injects into
         ▼
┌─────────────────────┐
│  SipHash-1-2 State   │  ← ARX mixing: full 64-bit avalanche
│  v0, v1, v2, v3     │
└────────┬────────────┘
         │  counter mode
         ▼
    keystream bytes
         │
         ▼
    plaintext ⊕ keystream = ciphertext
```

**The Collatz trajectory is the state driver, not the output.** Raw Collatz bits are statistically biased (this is documented — see Design Decisions below). The SipHash rounds completely decouple the output statistics from the trajectory, producing a clean, unbiased keystream. Encryption and decryption are symmetric (XOR).

---

## Installation

```bash
# From crates.io
cargo install lambda-shield

# From source
git clone https://github.com/0x-auth/lambda-shield
cd lambda-shield
cargo build --release
```

---

## Usage

### Encrypt a message

```bash
lambda-shield --msg <seed_hi> <seed_lo> "your message"

# Example
lambda-shield --msg 12345 67890 "Bazinga! Lemma 3 is live."
# Hex: [62, 8c, 53, ...]
# Dec: Bazinga! Lemma 3 is live.
```

Seeds are `u64` integers (decimal or `0x` hex). Both are required — together they form a 128-bit key.

### Encrypt a file

```bash
lambda-shield --file <seed_hi> <seed_lo> path/to/file
# Output saved as path/to/file.lambda
# Run again with same seeds to decrypt
```

### Generate NIST test keystream

```bash
lambda-shield --nist <seed_hi> <seed_lo>
# Generates keystream.bin (1MB binary) and keystream.txt (8M ASCII bits)
# Feed keystream.txt into NIST STS ./assess for full SP 800-22 report
```

---

## NIST SP 800-22 Results

Tested using the **official NIST STS v2.1.2 binary** (`./assess`) on 1MB keystream (seed `12345` / `67890`):

| Test Category | Tests Run | Passed | Result |
|:--------------|----------:|-------:|:------:|
| Frequency | 1 | 1 | ✅ |
| Block Frequency | 1 | 1 | ✅ |
| Cumulative Sums | 2 | 2 | ✅ |
| Runs | 1 | 1 | ✅ |
| Longest Run | 1 | 1 | ✅ |
| Rank | 1 | 1 | ✅ |
| FFT (Spectral) | 1 | 1 | ✅ |
| Non-Overlapping Template | 148 | 148 | ✅ |
| Overlapping Template | 1 | 1 | ✅ |
| Universal Statistical | 1 | 1 | ✅ |
| Approximate Entropy | 1 | 1 | ✅ |
| Serial | 2 | 2 | ✅ |
| Linear Complexity | 1 | 1 | ✅ |
| Random Excursions | — | — | ⏭ skipped* |
| Random Excursions Variant | — | — | ⏭ skipped* |
| **TOTAL** | **162** | **162** | ✅ **ALL PASS** |

**162/162 tests passed.** The keystream is statistically indistinguishable from random noise across the full NIST SP 800-22 battery.

*RandomExcursions tests require multiple sequences or longer input to accumulate sufficient excursions — skipping is standard behaviour with a single 1MB sequence, not a failure.

Reproduce:
```bash
lambda-shield --nist 12345 67890
# Feed keystream.bin into NIST STS v2.1.2:
# ./assess 1000000  →  select file input  →  keystream.bin
```

Full results: `experiments/AlgorithmTesting/finalAnalysisReport.txt`

---

## Design Decisions

### Why Collatz?

The Collatz map (`n → n/2` if even, `n → 3n+1` if odd) produces trajectories with no known closed-form, no algebraic shortcut, and empirically geometric distribution of step values. This makes it a good *entropy source* — unpredictable state evolution without complex operations.

**Important:** Raw Collatz bit output is biased. This was identified during development and is the reason for the SipHash mixing layer. The Collatz trajectory drives state, but never appears directly in the output.

### Why SipHash-1-2 rounds?

SipHash was designed for fast, secure hashing with full avalanche on constrained hardware. Its ARX (Add-Rotate-XOR) structure runs in a few cycles on any MCU without dedicated crypto hardware. Two rounds per block provide enough diffusion to break all statistical correlations from the Collatz input.

### Why 128-bit seed?

The 64-bit seed in v1 allowed brute-force recovery at ~3.5M seeds/sec on a laptop — feasible with parallelism. The 128-bit seed (`seed_hi` + `seed_lo`) provides a 2^128 keyspace, equivalent to AES-128.

### Operations used

Every operation in Lambda-Shield is from this set:

| Operation | MCU cost |
|:----------|:---------|
| `wrapping_add` | 1 cycle |
| `XOR` | 1 cycle |
| `rotate_left` | 1 cycle |
| integer divide by 2 (right shift) | 1 cycle |

No S-boxes, no lookup tables, no floating point. Runs on ARM Cortex-M0.

---

## Benchmark vs ChaCha20

Measured on x86-64 release build (`rustc -C opt-level=2`):

| Cipher | 1KB throughput | 64KB throughput |
|:-------|---------------:|----------------:|
| Lambda-Shield v4 | ~65 MiB/s | ~60 MiB/s |
| ChaCha20 (software) | ~1,300 MiB/s | ~1,300 MiB/s |

Lambda-Shield is ~20x slower than ChaCha20 on x86-64. **This is expected and intentional** — ChaCha20 is heavily optimised for desktop CPUs with SIMD. On a low-power MCU (ARM Cortex-M0, no SIMD, no AES-NI), the gap narrows significantly because ChaCha20 loses its SIMD advantage while Lambda-Shield's simple integer ops remain fast.

For IoT use cases where the bottleneck is sensor data rate (not CPU), throughput matters less than gate count and power draw per byte.

---

## Security Notes

- **This cipher has not undergone formal cryptanalysis.** It passes NIST SP 800-22 statistical tests, which is a necessary but not sufficient condition for security.
- **Do not reuse seeds.** Like all stream ciphers, reusing a (seed_hi, seed_lo) pair with different plaintexts breaks confidentiality.
- **Seeds are not keys in a cryptographic sense** until independent security review has been completed. For production use in sensitive systems, use a well-audited cipher (AES-GCM, ChaCha20-Poly1305).
- This is a research prototype with a novel construction. Treat it as such.

---

## Version History

| Version | Change |
|:--------|:-------|
| v4.0 | SipHash-1-2 mixing, 128-bit seed, counter mode, NIST tested |
| v3.0 | Double Murmur3 conditioning — fixed bias, periodicity remained |
| v2.0 | Single Murmur3 output conditioning — fixed keyspace, bias partially fixed |
| v1.0 | Raw Collatz XOR — 62% bit bias, 64-bit seed, periodic rekey events |

The v1→v4 journey is documented in detail in the associated research paper (Zenodo, 2025).

---

## Research

This cipher is part of ongoing work on Collatz residue distributions and their cryptographic properties.

- **Paper:** *Lambda-Shield v4: A Collatz-Trajectory Stream Cipher with SipHash-1-2 Mixing* — [Zenodo DOI: 10.5281/zenodo.18969011](https://zenodo.org/records/18969011)
- **ORCID:** [0009-0006-7495-5039](https://orcid.org/0009-0006-7495-5039)

---

## License

MIT — see [LICENSE](LICENSE)

---

*Built by [Abhishek Srivastava](https://github.com/0x-auth)*
