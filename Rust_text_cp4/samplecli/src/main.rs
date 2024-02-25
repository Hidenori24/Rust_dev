use anyhow::{bail, ensure, Context, Result};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

use clap::Parser;

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

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

/* run機能の追加 */
fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalcukator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => println!("{:#?}", e),
        }
    }
    Ok(())
}
struct RpnCalcukator(bool);

impl RpnCalcukator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            // '-v' オプションが指定されている場合は，この時点でトークンとスタックの状態を表示
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ok() {
        let calc = RpnCalcukator(false);
        // number only
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-5").unwrap(), -5);

        // fomula + numbers
        assert_eq!(calc.eval("4 5 +").unwrap(), 9);
        assert_eq!(calc.eval("5 12 -").unwrap(), -7);
        assert_eq!(calc.eval("2 7 *").unwrap(), 14);
        assert_eq!(calc.eval("6 2 /").unwrap(), 3);
        assert_eq!(calc.eval("13 6 %").unwrap(), 1);
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalcukator(false);

        assert!(calc.eval("3 5 ^").is_err());
        assert!(calc.eval("1 1 1 +").is_err());
        assert!(calc.eval("+ 1 1").is_err());
    }
}
