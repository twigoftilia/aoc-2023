#![allow(dead_code)]

use std::env::Args;

use clap::ArgAction;
use clap::{Arg, Command};

pub static FAIL_STRING: &str = "Could not solve task";
pub static PARSE_FAIL_STRING: &str = "Parsing error";

// Returns a vector of ints, taken from an aoc provided input file (one integer per row)
pub fn integer_rows_to_vector(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().expect("Not an integer"))
        .collect()
}

// Returns a vector of trimmed strings with row content
pub fn rows_to_vector(input: &str) -> Vec<&str> {
    input.lines().map(|line| line.trim()).collect()
}

// Returns a vector of vectors of ints from input. Empty spaces gives new vector
// i.e. 2022 day01
pub fn vec_of_integer_vec(input: &str) -> Vec<Vec<i32>> {
    let r = input
        .split("\n\n")
        .map(|lines| {
            lines.lines().fold(vec![], |mut acc, line| {
                acc.push(line.trim().parse().expect("Not an integer"));
                acc
            })
        })
        .collect();
    r
}

fn test_inclusive_range(str: &str, min: &i32, max: &i32) -> bool {
    //    dbg!(str);
    let i = str.parse::<i32>();
    if i.is_ok() {
        let i = i.unwrap();
        return i.ge(min) && i.le(max);
    }
    false
}

pub fn intersects(x1: usize, x2: usize, y1: usize, y2: usize) -> bool {
    x1 <= y2 && y1 <= x2
}


#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub const VALUES: [Self; 4] = [Self::North, Self::South, Self::East, Self::West];
}

pub fn parse_parts_to_run(args: Args) -> (bool, bool) {
    let matches = Command::new("prog")
        .arg(
            Arg::new("only_part_1")
                .short('1')
                .help("run only part 1")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("only_part_2")
                .short('2')
                .help("run only part 2")
                .action(ArgAction::SetTrue),
        )
        .get_matches_from(args);

    let mut only1 = matches.get_flag("only_part_1");
    let mut only2 = matches.get_flag("only_part_2");

    if only1 && only2 {
        only1 = false;
        only2 = false;
    }

    (!only2, !only1)
}
