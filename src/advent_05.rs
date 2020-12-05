pub struct Advent;

fn parse_ticket(spec: &str) -> u32 {
    if spec.len() != 10 {
        panic!();
    }
    ((spec.as_bytes()[0] == b'B') as u32 * 512)
        + ((spec.as_bytes()[1] == b'B') as u32 * 256)
        + ((spec.as_bytes()[2] == b'B') as u32 * 128)
        + ((spec.as_bytes()[3] == b'B') as u32 * 64)
        + ((spec.as_bytes()[4] == b'B') as u32 * 32)
        + ((spec.as_bytes()[5] == b'B') as u32 * 16)
        + ((spec.as_bytes()[6] == b'B') as u32 * 8)
        + ((spec.as_bytes()[7] == b'R') as u32 * 4)
        + ((spec.as_bytes()[8] == b'R') as u32 * 2)
        + ((spec.as_bytes()[9] == b'R') as u32 * 1)
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        5
    }

    fn main1(input: &String) -> String {
        format!("{}", input.lines().map(|x| parse_ticket(x)).max().unwrap())
    }

    fn main2(input: &String) -> String {
        let every_single_taken_seat = input
            .lines()
            .map(|x| parse_ticket(x))
            .collect::<std::collections::HashSet<_>>();
        for i in 1..998 {
            if every_single_taken_seat.contains(&(i - 1))
                && !every_single_taken_seat.contains(&i)
                && every_single_taken_seat.contains(&(i + 1))
            {
                return format!("{}", i);
            }
        }
        panic!();
    }
}
