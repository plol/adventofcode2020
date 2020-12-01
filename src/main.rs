#![allow(dead_code)]

use std;

mod common;
use common::read_file;

mod advent_01;

fn run_advent<A>(input: String, expected1: &str, expected2: &str)
where
    A: common::Advent,
{
    let before = std::time::Instant::now();
    let result1 = A::main1(&input);
    let middle = std::time::Instant::now();
    let result2 = A::main2(&input);
    let after = std::time::Instant::now();
    let dt1 = (middle - before).as_nanos() as f64 / 1000000.0;
    let dt2 = (after - middle).as_nanos() as f64 / 1000000.0;
    if result1 != expected1 {
        println!("part1 failed! Expected {} was {}", expected1, result1);
    }
    if result2 != expected2 {
        println!("part2 failed! Expected {} was {}", expected2, result2);
    }
    println!(
        "Advent {}: part1 {:.1} ms, part2 {:.1} ms",
        A::advent_number(),
        dt1,
        dt2
    );
}

fn main() {
    run_advent::<advent_01::Advent>(read_file("inputs/input01"), "1016131", "276432018");
}
