use std::env;
use std::fs;
use lambda_shield::lambda_process;

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
