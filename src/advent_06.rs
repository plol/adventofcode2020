pub struct Advent;

fn do_it<F>(input: &String, fold_seed: u32, fold_function: F) -> u32
where
    F: Fn(u32, u32) -> u32,
{
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|c| match c {
                            b'a'..=b'z' => c - b'a',
                            _ => 0,
                        })
                        .fold(0u32, |a, b| a | (1 << b))
                })
                .fold(fold_seed, &fold_function)
                .count_ones()
        })
        .sum()
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        6
    }

    fn main1(input: &String) -> String {
        format!("{}", do_it(input, 0, |a, b| a | b))
    }

    fn main2(input: &String) -> String {
        format!("{}", do_it(input, 0xFFFFFFFF, |a, b| a & b))
    }
}
