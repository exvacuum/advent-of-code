use std::{
    fs::File,
    io::{self, BufRead},
    isize,
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    pub input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_file = File::open(&args.input).unwrap();
    let mut safe_count = 0;
    'outer: for line in io::BufReader::new(input_file).lines().flatten() {
        let reports = line
            .split(" ")
            .map(|n| isize::from_str_radix(n, 10))
            .flatten()
            .collect::<Vec<_>>();
        for report_window in reports.windows(3) {
            let delta_ab = report_window[1] - report_window[0];
            let delta_bc = report_window[2] - report_window[1];
            if delta_ab.abs() > 3 || delta_bc.abs() > 3 || delta_ab.signum() != delta_bc.signum() {
                continue 'outer;
            }
        }
        safe_count += 1;
    }
    println!("{safe_count}");
}
