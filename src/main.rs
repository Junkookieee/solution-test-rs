use std::env;
use std::fs::File;
use std::time::Instant;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
     let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let input_file = match args.get(1) {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: {} <input_file>", args[0]);
            return Ok(());
        }
    };

    let output_file = match args.get(2) {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: {} <input_file> <output_file>", args[0]);
            return Ok(());
        }
    };

    // Open input file for reading
    let input = File::open(input_file)?;
    let reader = BufReader::new(input);

    // Open output file for writing
    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    // Process the file in chunks
    let mut chunk = Vec::new();
    let mut reader = reader.bytes();
    while let Some(Ok(byte)) = reader.next() {
        chunk.push(byte);
        if chunk.len() >= 1024 * 1024 {
            // Process the current chunk
            let chunk_str = String::from_utf8_lossy(&chunk);
            let corrected_chunk = chunk_str.replace(";", ":");
            writer.write_all(corrected_chunk.as_bytes())?;
            chunk.clear();
        }
    }

    // Process the last chunk
    if !chunk.is_empty() {
        let chunk_str = String::from_utf8_lossy(&chunk);
        let corrected_chunk = chunk_str.replace(";", ":");
        writer.write_all(corrected_chunk.as_bytes())?;
    }

    writer.flush()?;
    println!("File processed successfully.");
    println!("Time elapsed in processing file is: {:?}", now.elapsed());
    Ok(())
}
