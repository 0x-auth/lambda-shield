#!/bin/bash

echo "🚀 Starting Lambda-Shield Verification..."

# 1. Compile
echo "🔨 Compiling main and checker..."
rustc src/main.rs -o lambda_shield
rustc src/checker.rs -o checker

# 2. Test Entropy
echo "📊 Running Lemma 3 Entropy Check..."
./checker 123456789

# 3. Test Encryption/Decryption
echo "🔒 Testing Cipher Logic..."
SAMPLE_MSG="Bazinga! Lemma 3 test successful."
RESULT=$(./lambda_shield --msg 123456789 "$SAMPLE_MSG")

if [[ "$RESULT" == *"$SAMPLE_MSG"* ]]; then
    echo "✅ Success: Decrypted message matches original!"
else
    echo "❌ Error: Decryption mismatch."
    exit 1
fi

echo "🏁 All tests passed!"
