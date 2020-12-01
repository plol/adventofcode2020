pub struct Advent;

use super::common::parse_lines;

fn find_indices(what: &Vec<i64>) -> Option<(usize, usize)> {
    for i in 0..what.len() {
        for j in i + 1..what.len() {
            if what[i] + what[j] == 2020 {
                return Some((i, j));
            }
        }
    }
    None
}
fn find_indices2(what: &Vec<i64>) -> Option<(usize, usize, usize)> {
    for i in 0..what.len() {
        for j in i + 1..what.len() {
            for k in j + 1..what.len() {
                if what[i] + what[j] + what[k] == 2020 {
                    return Some((i, j, k));
                }
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
        let (i, j) = find_indices(&what).unwrap();
        format!("{}", what[i] * what[j])
    }

    fn main2(input: &String) -> String {
        let what = parse_lines(input);
        let (i, j, k) = find_indices2(&what).unwrap();
        format!("{}", what[i] * what[j] * what[k])
    }
}
