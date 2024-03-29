#![allow(dead_code)]

extern crate codegen_stuff;

use std;

mod common;
use common::read_file;

mod advent_01;
mod advent_02;
mod advent_03;
mod advent_04;
mod advent_05;
mod advent_06;
mod advent_07;
mod advent_08;
mod advent_09;
mod advent_10;
mod advent_11;
mod advent_12;
mod advent_13;
mod advent_14;
mod advent_15;
mod advent_16;
mod advent_17;

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
    run_advent::<advent_02::Advent>(read_file("inputs/input02"), "398", "562");
    run_advent::<advent_03::Advent>(read_file("inputs/input03"), "167", "736527114");
    run_advent::<advent_04::Advent>(read_file("inputs/input04"), "228", "175");
    run_advent::<advent_05::Advent>(read_file("inputs/input05"), "998", "676");
    run_advent::<advent_06::Advent>(read_file("inputs/input06"), "6590", "3288");
    run_advent::<advent_07::Advent>(read_file("inputs/input07"), "278", "45157");
    run_advent::<advent_08::Advent>(read_file("inputs/input08"), "1420", "1245");
    run_advent::<advent_09::Advent>(read_file("inputs/input09"), "138879426", "23761694");
    run_advent::<advent_10::Advent>(read_file("inputs/input10"), "3000", "193434623148032");
    run_advent::<advent_11::Advent>(read_file("inputs/input11"), "2164", "1974");
    run_advent::<advent_12::Advent>(read_file("inputs/input12"), "582", "52069");
    run_advent::<advent_13::Advent>(read_file("inputs/input13"), "104", "842186186521918");
    run_advent::<advent_14::Advent>(
        read_file("inputs/input14"),
        "5875750429995",
        "5272149590143",
    );
    run_advent::<advent_15::Advent>("1,20,8,12,0,14".to_owned(), "492", "63644");
    run_advent::<advent_16::Advent>(read_file("inputs/input16"), "21956", "3709435214239");
    run_advent::<advent_17::Advent>(read_file("inputs/input17"), "21956", "3709435214239");
}
