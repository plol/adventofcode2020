pub struct Advent;

use super::common::parse_lines;

fn find_values(what: &Vec<i64>) -> Option<(i64, i64)> {
    let values = what.iter().collect::<std::collections::HashSet<_>>();
    for x in what.iter() {
        if values.contains(&(2020 - x)) {
            return Some((*x, 2020 - x));
        }
    }
    None
}

fn find_values2(what: &Vec<i64>) -> Option<(i64, i64, i64)> {
    let values = what.iter().collect::<std::collections::HashSet<_>>();
    for i in 0..what.len() {
        let x = what[i];
        for y in what[i + 1..].iter() {
            if values.contains(&(2020 - x - y)) {
                return Some((x, *y, 2020 - x - y));
            }
        }
    }
    None
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        1
    }

    fn main1(input: &String) -> String {
        let what = parse_lines(input);
        let (x, y) = find_values(&what).unwrap();
        format!("{}", x * y)
    }

    fn main2(input: &String) -> String {
        let what = parse_lines(input);
        let (x, y, z) = find_values2(&what).unwrap();
        format!("{}", x * y * z)
    }
}
