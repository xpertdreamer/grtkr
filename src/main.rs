use clap::Parser;

#[derive(Parser)]
struct CLI {
    // Which pattern look for
    pat: String,
    // A path to the file
    file: std::path::PathBuf
}

fn main() {
    let args = CLI::parse();
    println!("pat: {:?}, file: {:?}", args.pat, args.file);
}
