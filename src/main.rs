use std::env;
use std::fs;

fn lambda_process(data: &[u8], seed: u64) -> Vec<u8> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./lambda_shield --msg <seed> \"text\" OR --file <seed> <path>");
        return;
    }
    let mode = &args[1];
    let seed: u64 = args[2].parse().unwrap_or(703);
    let raw_data: Vec<u8> = match mode.as_str() {
        "--msg" => args[3].as_bytes().to_vec(),
        "--file" => fs::read(&args[3]).expect("Could not read file"),
        _ => return,
    };
    let processed = lambda_process(&raw_data, seed);
    if mode == "--msg" {
        println!("--- Lambda Shield Output ---");
        println!("Original:  {}", String::from_utf8_lossy(&raw_data));
        println!("Hex:       {:02x?}", processed);
        println!("Decrypted: {}", String::from_utf8_lossy(&lambda_process(&processed, seed)));
    } else {
        let out_path = format!("{}.lambda", args[3]);
        fs::write(&out_path, &processed).expect("Failed to write output");
        println!("File processed. Output saved to: {}", out_path);
    }
}
