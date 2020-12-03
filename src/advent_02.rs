pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        2
    }

    fn main1(input: &String) -> String {
        format!(
            "{}",
            input
                .lines()
                .filter(|line| {
                    let (n1, n2, letter, password): (usize, usize, char, String) =
                        codegen_stuff::scan!("{}-{} {}: {}", line);
                    let count = password.chars().filter(|x| *x == letter).count();
                    n1 <= count && count <= n2
                })
                .count()
        )
    }

    fn main2(input: &String) -> String {
        format!(
            "{}",
            input
                .lines()
                .filter(|line| {
                    let (n1, n2, letter, password): (usize, usize, char, String) =
                        codegen_stuff::scan!("{}-{} {}: {}", line);
                    (password.chars().nth(n1 - 1).unwrap() == letter)
                        != (password.chars().nth(n2 - 1).unwrap() == letter)
                })
                .count()
        )
    }
}
