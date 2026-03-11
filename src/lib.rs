/// Lambda-Shield Core Logic: A Lemma 3 based Stream Cipher.
pub fn lambda_process(data: &[u8], seed: u64) -> Vec<u8> {
    let mut n: u128 = if seed == 0 { 42 } else { seed as u128 };
    let mut result = Vec::with_capacity(data.len());

    for &byte in data {
        let mut keystream_byte: u8 = 0;
        for i in 0..8 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = n.wrapping_mul(3).wrapping_add(1);
                keystream_byte |= 1 << i;
            }
            if n <= 1 {
                n = (seed as u128).wrapping_add(keystream_byte as u128).wrapping_add(i as u128);
            }
            if n > (u64::MAX as u128) {
                n = (n % (u64::MAX as u128)) + 7;
            }
        }
        result.push(byte ^ keystream_byte);
    }
    result
}
