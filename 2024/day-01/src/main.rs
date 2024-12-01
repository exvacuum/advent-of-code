use std::{collections::HashMap, fs::File, io::{self, BufRead}, path::PathBuf, usize};

use clap::Parser;
use scan_rules::scan;
use scan_rules::scan_rules_impl;

#[derive(Parser)]
struct Args {
    pub input: PathBuf,
    #[clap(short='2')]
    pub part_2: bool,
}

fn main() {
    let args = Args::parse();
    if args.part_2 {
        part_2(&args);
    } else {
        part_1(&args);
    }
}

fn part_1(args: &Args) {
    let input_file = File::open(&args.input).unwrap();
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

fn part_2(args: &Args) {
    let input_file = File::open(&args.input).unwrap();
    let mut list_a = vec![];
    let mut list_b_map = HashMap::<usize, usize>::new();
    for line in io::BufReader::new(input_file).lines().flatten() {
        scan! {
            &line;
            (let a: usize, "   ", let b: usize) => {
                list_a.push(a);
                list_b_map.entry(b).and_modify(|counter| *counter += 1).or_insert(1);
            },
        }.unwrap();
    }
    let mut similarity_score = 0;
    for a in list_a.into_iter() {
        similarity_score += a * list_b_map.get(&a).unwrap_or(&0);
    }
    println!("{similarity_score}");
}
