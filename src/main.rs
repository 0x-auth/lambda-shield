use std::env;
use std::fs;
use lambda_shield::{lambda_process, lambda_keystream};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage:");
        println!("  --msg  <hi> <lo> \"text\"");
        println!("  --file <hi> <lo> <path>");
        println!("  --nist <hi> <lo>");
        return;
    }

    let mode = &args[1];
    let hi: u64 = args[2].parse().expect("Invalid seed_hi");
    let lo: u64 = args[3].parse().expect("Invalid seed_lo");

    match mode.as_str() {
        "--msg" => {
            let msg = args.get(4).expect("Missing message").as_bytes();
            let enc = lambda_process(msg, hi, lo);
            println!("Hex: {:02x?}", enc);
            let dec = lambda_process(&enc, hi, lo);
            println!("Dec: {}", String::from_utf8_lossy(&dec));
        }
        "--file" => {
            let path = args.get(4).expect("Missing path");
            let data = fs::read(path).expect("Read failed");
            let enc = lambda_process(&data, hi, lo);
            fs::write(format!("{}.lambda", path), &enc).unwrap();
            println!("Processed {} bytes to {}.lambda", data.len(), path);
        }
        "--nist" => {
            let ks = lambda_keystream(1_000_000, hi, lo);
            fs::write("keystream.bin", &ks).unwrap();
            println!("Generated 1MB keystream.bin for NIST testing.");
        }
        _ => println!("Unknown mode"),
    }
}
