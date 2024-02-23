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
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }

            // '-v' オプションが指定されている場合は，この時点でトークンとスタックの状態を表示
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ok() {
        let calc = RpnCalcukator(false);
        // number only
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("50"), 50);
        assert_eq!(calc.eval("-5"), -5);

        // fomula + numbers
        assert_eq!(calc.eval("4 5 +"), 9);
        assert_eq!(calc.eval("5 12 -"), -7);
        assert_eq!(calc.eval("2 7 *"), 14);
        assert_eq!(calc.eval("6 2 /"), 3);
        assert_eq!(calc.eval("13 6 %"), 1);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalcukator(false);

        calc.eval("3 5 ^");
    }
}
