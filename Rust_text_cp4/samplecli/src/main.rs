use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

use clap::Parser; // change

#[derive(Parser, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "John Smith",
    about = "Super awesome sample RPN calclator"
)]

struct Opts {
    // Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

/* run機能の追加 */
fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalcukator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}
struct RpnCalcukator(bool);

impl RpnCalcukator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        0
    }
}
