use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::PathBuf, usize};

use clap::Parser;
use regex::Regex;
use itertools::Itertools;



#[derive(Parser)]
struct Args {
    pub input: PathBuf,
}

fn main() {
    let args = Args::parse();

    let mut ruleset = HashSet::<(usize, usize)>::new();

    let input_file = File::open(&args.input).unwrap();
    let mut input_lines = BufReader::new(input_file).lines().flatten();

    loop {
        let line = match input_lines.next() {
            Some(line) => line,
            None => break,
        };
        let rule_regex = Regex::new(r#"^(\d+)\|(\d+)$"#).unwrap();
        let mut captures = rule_regex.captures_iter(&line).map(|c| c.extract());
        if let Some((_, [a, b])) = captures.next() {
            ruleset.insert((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
        } else {
            break;
        }
    }

    let mut sum = 0;
    'outer: while let Some(line) = input_lines.next() {
        if let Ok(numbers) = line.split(',').map(|n| n.parse::<usize>()).collect::<Result<Vec<_>,_>>() {
            for combo in numbers.iter().combinations(2) {
                for rule in ruleset.iter() {
                    if *combo[0] == rule.1 && *combo[1] == rule.0 {
                        continue 'outer;
                    }
                }
            }
            let middle_index = numbers.len() / 2;
            sum += numbers[middle_index];
        }
    }

    println!("{sum}");
}
