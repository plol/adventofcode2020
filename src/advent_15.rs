pub struct Advent;

fn game(data: Vec<usize>, n: usize) -> usize {
    let mut last_seen = vec![0; n];
    for i in 0..data.len() - 1 {
        last_seen[data[i]] = i + 1;
    }
    let mut prev = data[data.len() - 1];
    for i in data.len()..n {
        let j = last_seen[prev];
        let next = if j == 0 { 0 } else { i - j };
        last_seen[prev] = i;
        prev = next;
    }
    prev
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        15
    }

    fn main1(input: &String) -> String {
        format!(
            "{}",
            game(input.split(",").map(|x| x.parse().unwrap()).collect(), 2020)
        )
    }

    fn main2(input: &String) -> String {
        format!(
            "{}",
            game(
                input.split(",").map(|x| x.parse().unwrap()).collect(),
                30000000
            )
        )
    }
}
