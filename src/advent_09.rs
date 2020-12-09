pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        9
    }

    fn main1(input: &String) -> String {
        let data = input
            .lines()
            .map(|line| codegen_stuff::scan!("{}", line))
            .collect::<Vec<i64>>();
        let mut working_set = std::collections::HashSet::new();
        working_set.extend(data[..25].iter());
        for i in 25..data.len() {
            let x = data[i];
            if !working_set.iter().any(|y| working_set.contains(&(x - y))) {
                return format!("{}", x);
            }
            working_set.remove(&data[i - 25]);
            working_set.insert(x);
        }
        panic!();
    }

    fn main2(input: &String) -> String {
        let data = input
            .lines()
            .map(|line| codegen_stuff::scan!("{}", line))
            .collect::<Vec<i64>>();

        let mut range_start = 0;
        let mut range_end = 0;
        let mut sum = 0;

        let target = 138879426;

        while sum != target {
            if sum < target {
                sum += data[range_end];
                range_end += 1;
            } else {
                sum -= data[range_start];
                range_start += 1;
            }
        }

        let range = &data[range_start..range_end];

        format!(
            "{}",
            range.iter().min().unwrap() + range.iter().max().unwrap()
        )
    }
}
