#!/bin/bash

echo "🚀 Starting Lambda-Shield v2 Verification..."

# 1. Build
cargo build --release

# 2. Test Entropy (Checker)
rustc src/checker.rs -o checker
echo "📊 Running Lemma 3 Entropy Check..."
./checker 123456789

# 3. Test Encryption/Decryption with TWO seeds
echo "🔒 Testing Cipher Logic (Symmetric v2)..."
SAMPLE_MSG="Bazinga! Lemma 3 v2 is live."

# Execution with 5 args: [0]bin [1]--msg [2]seed_hi [3]seed_lo [4]message
RESULT=$(./target/release/lambda-shield --msg 12345 67890 "$SAMPLE_MSG")

if [[ "$RESULT" == *"$SAMPLE_MSG"* ]]; then
    echo "✅ Success: Decrypted message matches original!"
    echo "$RESULT"
else
    echo "❌ Error: Decryption mismatch or Panic."
    echo "Output was: $RESULT"
    exit 1
fi

echo "🏁 All v2 tests passed!"
