pub struct Advent;

fn parse(input: &String) -> Vec<Vec<(&str, &str)>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_terminator(|c: char| c.is_ascii_whitespace())
                .map(|p| codegen_stuff::scan_strs!("{}:{}", p))
                .collect()
        })
        .collect()
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" => "1920" <= value && value <= "2002",
        "iyr" => "2010" <= value && value <= "2020",
        "eyr" => "2020" <= value && value <= "2030",
        "hgt" => {
            value.ends_with("cm") && "150cm" <= value && value <= "193cm"
                || value.ends_with("in") && "59in" <= value && value <= "76in"
        }
        "hcl" => value.starts_with("#") && u64::from_str_radix(&value["#".len()..], 16).is_ok(),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.parse::<u64>().is_ok(),
        _ => true,
    }
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        4
    }

    fn main1(input: &String) -> String {
        let passports = parse(input);
        format!(
            "{}",
            passports
                .iter()
                .filter(|passport| REQUIRED_FIELDS
                    .iter()
                    .all(|field| passport.iter().any(|r| r.0 == *field)))
                .count()
        )
    }

    fn main2(input: &String) -> String {
        let passports = parse(input);
        format!(
            "{}",
            passports
                .iter()
                .filter(|passport| REQUIRED_FIELDS.iter().all(|field| passport
                    .iter()
                    .any(|r| r.0 == *field && validate(field, r.1))))
                .count()
        )
    }
}
