use std::env;
use lambda_shield::lambda_shield_v2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: ./lambda_shield --msg <seed_hi> <seed_lo> \"message\"");
        return;
    }

    let seed_hi: u64 = args[2].parse().unwrap_or(0);
    let seed_lo: u64 = args[3].parse().unwrap_or(0);
    let msg = args[4].as_bytes();

    let encrypted = lambda_shield_v2(msg, seed_hi, seed_lo);
    println!("--- Lambda Shield v2 Output ---");
    println!("Encrypted (Hex): {:02x?}", encrypted);

    let decrypted = lambda_shield_v2(&encrypted, seed_hi, seed_lo);
    println!("Decrypted:       {}", String::from_utf8_lossy(&decrypted));
}
use std::env;
use lambda_shield::lambda_shield_v2;

fn main() {
    let args: Vec<String> = env::args().collect();
    // We need: executable, mode, seed_hi, seed_lo, message (5 args)
    if args.len() < 5 {
        println!("Usage: ./lambda_shield --msg <seed_hi> <seed_lo> \"message\"");
        return;
    }

    let mode = &args[1];
    let seed_hi: u64 = args[2].parse().unwrap_or(0);
    let seed_lo: u64 = args[3].parse().unwrap_or(0);
    let data_str = &args[4];

    if mode == "--msg" {
        let raw_data = data_str.as_bytes();
        let encrypted = lambda_shield_v2(raw_data, seed_hi, seed_lo);
        
        println!("--- Lambda Shield v2 Output ---");
        println!("Original:  {}", data_str);
        println!("Hex:       {:02x?}", encrypted);
        
        let decrypted = lambda_shield_v2(&encrypted, seed_hi, seed_lo);
        println!("Decrypted: {}", String::from_utf8_lossy(&decrypted));
    }
}
