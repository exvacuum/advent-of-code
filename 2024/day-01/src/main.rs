use std::{fs::File, io::{self, BufRead}, path::PathBuf};

use clap::Parser;
use scan_rules::scan;
use scan_rules::scan_rules_impl;

#[derive(Parser)]
struct Args {
    pub input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_file = File::open(args.input).unwrap();
    let mut list_a_sorted = vec![];
    let mut list_b_sorted = vec![];
    for line in io::BufReader::new(input_file).lines().flatten() {
        scan! {
            &line;
            (let a: isize, "   ", let b: isize) => {
                  list_a_sorted.push(a);    
                  list_b_sorted.push(b);    
            },
        }.unwrap();
    }
    list_a_sorted.sort();
    list_b_sorted.sort();

    let mut sum = 0;
    for (a, b) in list_a_sorted.into_iter().zip(list_b_sorted.into_iter()) {
        sum += a.abs_diff(b) as usize;
    }
    println!("{sum}");
}
