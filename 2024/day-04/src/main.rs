use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize,
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    pub input: PathBuf,
    #[clap(short = '2')]
    pub part_2: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.input).unwrap();

    let word_search_grid = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut found_count = 0;
    for (row, char_row) in word_search_grid.iter().enumerate() {
        for (col, char) in char_row.iter().enumerate() {
            if args.part_2 {
                if *char == 'A' {
                    if search_for_an_xmas(&word_search_grid, row, col) {
                        found_count += 1;
                    }
                }
            } else {
                if *char == 'X' {
                    found_count += search_for_xmas(&word_search_grid, row, col);
                }
            }
        }
    }
    println!("{found_count}");
}

fn search_for_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut result = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if search_in_direction(grid, col, row, dx, dy) {
                result += 1;
            }
        }
    }
    result
}

fn search_in_direction(grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    for i in 1..XMAS.len() {
        let y_index = y as isize + dy * i as isize;
        let x_index = x as isize + dx * i as isize;

        if y_index.signum() == -1
            || x_index.signum() == -1
            || y_index as usize >= grid.len()
            || x_index as usize >= grid[0].len()
        {
            return false;
        }

        if grid[y_index as usize][x_index as usize] != XMAS[i] {
            return false;
        }
    }
    true
}

fn search_for_an_xmas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if row < 1 || row > grid.len() - 2 || col < 1 || col > grid[0].len() - 2 {
        return false;
    }
    let mut count = 0;
    for dy in [-1, 1] {
        for dx in [-1, 1] {
            if grid[(row as isize + dy) as usize][(col as isize + dx) as usize] == 'M'
                && grid[(row as isize - dy) as usize][(col as isize - dx) as usize] == 'S' {
                count += 1;
            }
        }
    }
    count == 2
}
