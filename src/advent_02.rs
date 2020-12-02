pub struct Advent;

#[derive(Debug)]
struct SpecAndPassword {
    n1: usize,
    n2: usize,
    letter: char,
    password: String,
}

impl std::str::FromStr for SpecAndPassword {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [range, letter_and_colon, passwordstr] = s.split(' ').collect::<Vec<_>>()[..] {
            if let [n1str, n2str] = range.split('-').collect::<Vec<_>>()[..] {
                if let Ok(n1) = n1str.parse::<usize>() {
                    if let Ok(n2) = n2str.parse() {
                        if let Some(letter) = letter_and_colon.chars().nth(0) {
                            return Ok(SpecAndPassword {
                                n1: n1,
                                n2: n2,
                                letter: letter,
                                password: passwordstr.to_string(),
                            });
                        }
                    }
                }
            }
        }
        Err("No".to_string())
    }
}

fn validate1(spec_and_password: &SpecAndPassword) -> bool {
    let count = spec_and_password
        .password
        .chars()
        .filter(|x| *x == spec_and_password.letter)
        .count();
    spec_and_password.n1 <= count && count <= spec_and_password.n2
}

fn validate2(spec_and_password: &SpecAndPassword) -> bool {
    (spec_and_password
        .password
        .chars()
        .nth(spec_and_password.n1 - 1)
        .unwrap()
        == spec_and_password.letter)
        != (spec_and_password
            .password
            .chars()
            .nth(spec_and_password.n2 - 1)
            .unwrap()
            == spec_and_password.letter)
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        2
    }

    fn main1(input: &String) -> String {
        let parsed_input = super::common::parse_lines(input);
        format!("{}", parsed_input.iter().filter(|sp| validate1(sp)).count())
    }

    fn main2(input: &String) -> String {
        let parsed_input = super::common::parse_lines(input);
        format!("{}", parsed_input.iter().filter(|sp| validate2(sp)).count())
    }
}
