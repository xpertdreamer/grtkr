use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

#[derive(Parser)]
struct CLI {
    // Which pattern look for
    pat: String,
    // A path to the file
    file: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLI::parse();
    // TODO: no panic
    let f = File::open(&args.file).expect("couldn`t open a file");
    let reader = BufReader::new(f);

    // TODO: print some patterns like line-numbers, dollar-signs and etc
    for ln in reader.lines()
    {
        let line = ln?;
        let line = line.trim_end();
        println!("{} {}", args.pat, line);
    }

    // TODO: file statistics
    println!("pat: {:?}, file: {:?}", args.pat, args.file);
    Ok(())
}
