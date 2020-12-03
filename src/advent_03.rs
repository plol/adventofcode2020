pub struct Advent;

fn parse(input: &String) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn look_for_trees(slope: &Vec<Vec<u8>>, path: (usize, usize)) -> i64 {
    let mut pos = (0, 0);
    let mut trees_encountered = 0;
    while pos.1 < slope.len() {
        if slope[pos.1][pos.0 % slope[pos.1].len()] == b'#' {
            trees_encountered += 1;
        }
        pos = (pos.0 + path.0, pos.1 + path.1);
    }
    trees_encountered
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        3
    }

    fn main1(input: &String) -> String {
        format!("{}", look_for_trees(&parse(input), (3, 1)))
    }

    fn main2(input: &String) -> String {
        let parsed_input = parse(&input);
        format!(
            "{}",
            vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
                .drain(..)
                .map(|path| look_for_trees(&parsed_input, path))
                .product::<i64>()
        )
    }
}
