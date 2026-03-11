#!/bin/bash

echo "🚀 Starting Lambda-Shield Verification via Cargo..."

# 1. Build the whole project using Cargo
echo "🔨 Building project..."
cargo build --release

# 2. Compile checker separately (since it's a standalone tool)
echo "🔨 Compiling checker..."
rustc src/checker.rs -o checker

# 3. Test Entropy
echo "📊 Running Lemma 3 Entropy Check..."
./checker 123456789

# 4. Test Encryption/Decryption using the Cargo-built binary
echo "🔒 Testing Cipher Logic..."
SAMPLE_MSG="Bazinga! Lemma 3 test successful."
# Using the binary built by cargo in the target folder
RESULT=$(./target/release/lambda-shield --msg 123456789 "$SAMPLE_MSG")

if [[ "$RESULT" == *"$SAMPLE_MSG"* ]]; then
    echo "✅ Success: Decrypted message matches original!"
else
    echo "❌ Error: Decryption mismatch."
    exit 1
fi

echo "🏁 All tests passed!"
