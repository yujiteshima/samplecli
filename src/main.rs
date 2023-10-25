use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program\
    ",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator",
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}
fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No file is specified");
    }
}
