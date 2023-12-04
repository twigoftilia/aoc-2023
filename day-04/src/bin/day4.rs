use std::env;

use aoc_lib::parse_parts_to_run;
use day_04::process_p1;
use day_04::process_p2;

fn main() {
    let parts_to_run = parse_parts_to_run(env::args());

    let file = include_str!("../../input.txt");
    if parts_to_run.0 {
        let result = process_p1(file).unwrap();
        println!("Part 1:\n{}", result);
    }

    if parts_to_run.1 {
        let result_p2 = process_p2(file).unwrap();
        println!("Part 2:\n{}", result_p2);
    }
}
