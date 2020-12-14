pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        12
    }

    fn main1(input: &String) -> String {
        let data = input
            .lines()
            .map(|line| (line.as_bytes()[0], line[1..].parse().unwrap()))
            .collect::<Vec<(u8, i32)>>();

        let mut pos = (0, 0);
        let mut dir = (1, 0);
        for cmd in data {
            match cmd {
                (b'F', n) => pos = (pos.0 + dir.0 * n, pos.1 + dir.1 * n),
                (b'N', n) => pos = (pos.0, pos.1 + n),
                (b'E', n) => pos = (pos.0 + n, pos.1),
                (b'S', n) => pos = (pos.0, pos.1 - n),
                (b'W', n) => pos = (pos.0 - n, pos.1),
                (b'R', 90) => dir = (dir.1, -dir.0),
                (b'R', 180) => dir = (-dir.0, -dir.1),
                (b'R', 270) => dir = (-dir.1, dir.0),
                (b'L', 90) => dir = (-dir.1, dir.0),
                (b'L', 180) => dir = (-dir.0, -dir.1),
                (b'L', 270) => dir = (dir.1, -dir.0),
                (a, b) => panic!("{}{}", a as char, b),
            }
        }
        format!("{}", pos.0.abs() + pos.1.abs())
    }

    fn main2(input: &String) -> String {
        let data = input
            .lines()
            .map(|line| (line.as_bytes()[0], line[1..].parse().unwrap()))
            .collect::<Vec<(u8, i32)>>();

        let mut pos = (0, 0);
        let mut wp = (10, 1);
        for cmd in data {
            match cmd {
                (b'F', n) => pos = (pos.0 + wp.0 * n, pos.1 + wp.1 * n),
                (b'N', n) => wp = (wp.0, wp.1 + n),
                (b'E', n) => wp = (wp.0 + n, wp.1),
                (b'S', n) => wp = (wp.0, wp.1 - n),
                (b'W', n) => wp = (wp.0 - n, wp.1),
                (b'R', 90) | (b'L', 270) => wp = (wp.1, -wp.0),
                (b'L', 90) | (b'R', 270) => wp = (-wp.1, wp.0),
                (b'R', 180) | (b'L', 180) => wp = (-wp.0, -wp.1),
                (a, b) => panic!("{}{}", a as char, b),
            }
        }
        format!("{}", pos.0.abs() + pos.1.abs())
    }
}
