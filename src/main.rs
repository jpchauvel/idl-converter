use std::env;
use std::fs::{self};
use std::io::Read;
use serde_json;
use solana_idl_converter::anchor_to_classic::try_convert;

fn main() {
    // Get the filename from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <idl-file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Read the IDL file
    let mut contents = String::new();
    if let Err(e) = fs::File::open(filename).and_then(|mut file| file.read_to_string(&mut contents)) {
        eprintln!("Failed to read file {}: {}", filename, e);
        std::process::exit(1);
    }

    // Deserialize the contents into an Idl structure
    let idl = match serde_json::from_str(&contents) {
        Ok(parsed_idl) => parsed_idl,
        Err(e) => {
            eprintln!("Failed to deserialize IDL: {}", e);
            std::process::exit(1);
        }
    };

    // Convert using solana_idl_converter
    let result = try_convert(idl);

    // Handle the result and overwrite the original file with the modified IDL
    match result {
        Ok(modified_idl) => {
            let json_output = match serde_json::to_string_pretty(&modified_idl) {
                Ok(output) => output,
                Err(e) => {
                    eprintln!("Failed to serialize modified IDL: {}", e);
                    std::process::exit(1);
                }
            };

            // Write the modified content back to the original file
            if let Err(e) = fs::write(filename, json_output) {
                eprintln!("Failed to write to file {}: {}", filename, e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error converting IDL: {:?}", e);
            std::process::exit(1);
        }
    }
}
