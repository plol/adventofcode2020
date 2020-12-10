pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        10
    }

    fn main1(input: &String) -> String {
        let mut data = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<i32>>();
        data.push(0);
        data.sort();
        let mut ones = 0;
        let mut threes = 1;
        for w in data.windows(2) {
            match w[1] - w[0] {
                1 => ones += 1,
                2 => {}
                3 => threes += 1,
                _ => panic!(),
            }
        }
        format!("{}", threes * ones)
    }

    fn main2(input: &String) -> String {
        let mut data = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<i32>>();
        data.push(0);
        data.sort();

        let mut c = vec![0u64; data.len()];
        c[data.len() - 1] = 1;

        for i in (0..data.len() - 1).rev() {
            for j in ((i + 1)..data.len()).take_while(|&j| data[j] <= data[i] + 3) {
                c[i] += c[j];
            }
        }

        format!("{}", c[0])
    }
}
