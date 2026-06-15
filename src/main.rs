use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;


#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
enum PAT_KIND
{
    DIGIT,
    DOLLAR,
    EMPTY
}

#[derive(Parser)]
struct CLI {
    // Which pattern look for
    pat: String,
    // A path to the file
    file: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;
    let args = CLI::parse();
    let pattern = match args.pat.as_str() {
        "digit" => PAT_KIND::DIGIT,
        "dollar" => PAT_KIND::DOLLAR,
        _ => PAT_KIND::EMPTY,
    };

    let f = File::open(&args.file);
    let file = match f {
        Ok(file) => { file },
        Err(error) => { return Err(error.into()); }
    };

    let reader = BufReader::new(file);

    for ln in reader.lines()
    {
        let line = ln?;
        let line = line.trim_end();

        match pattern {
            PAT_KIND::DIGIT => {
                println!("{}| {}", count + 1, line);
            }
            PAT_KIND::DOLLAR => {
                println!("$ {}", line);
            }
            PAT_KIND::EMPTY => {
                println!("{}", line);
            }
        }
        count += 1;
    }

    println!("\npattern: {:?}\npath: {:?}\nlines: {:?}", pattern, args.file, count);
    Ok(())
}
