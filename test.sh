#!/bin/bash
echo "🚀 Testing Lambda-Shield v4..."
cargo build --release
SAMPLE="Bazinga! Lemma 3 v4 is active."
# Passing TWO seeds: 123456789 and 987654321
RESULT=$(./target/release/lambda-shield --msg 123456789 987654321 "$SAMPLE")

if [[ "$RESULT" == *"$SAMPLE"* ]]; then
    echo "✅ v4 Logic Verified!"
    echo "$RESULT"
else
    echo "❌ v4 Logic Failed or Panicked!"
    exit 1
fi
