pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        16
    }

    fn main1(input: &String) -> String {
        let sections = input.split("\n\n").collect::<Vec<_>>();

        let spec = sections[0]
            .lines()
            .map(|line| codegen_stuff::scan!("{}: {}-{} or {}-{}", line))
            .collect::<Vec<(String, i32, i32, i32, i32)>>();

        let nearby_tickets = sections[2]
            .lines()
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();

        format!(
            "{}",
            nearby_tickets
                .iter()
                .map(|t| {
                    t.iter()
                        .filter(|&x| {
                            !spec
                                .iter()
                                .any(|(_, a1, a2, b1, b2)| a1 <= x && x <= a2 || b1 <= x && x <= b2)
                        })
                        .sum::<i32>()
                })
                .sum::<i32>()
        )
    }

    fn main2(input: &String) -> String {
        let sections = input.split("\n\n").collect::<Vec<_>>();

        let spec = sections[0]
            .lines()
            .map(|line| codegen_stuff::scan!("{}: {}-{} or {}-{}", line))
            .collect::<Vec<(String, i32, i32, i32, i32)>>();

        let my_ticket = sections[1]
            .lines()
            .nth(1)
            .map(|line| {
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .unwrap();

        let nearby_tickets = sections[2]
            .lines()
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();

        let mut columns = vec![vec![]; my_ticket.len()];

        for valid_ticket in nearby_tickets.iter().filter(|t| {
            !t.iter().any(|x| {
                !spec
                    .iter()
                    .any(|(_, a1, a2, b1, b2)| a1 <= x && x <= a2 || b1 <= x && x <= b2)
            })
        }) {
            for i in 0..columns.len() {
                columns[i].push(valid_ticket[i]);
            }
        }

        let mut valid_interpretations = columns
            .iter()
            .map(|column| {
                spec.iter()
                    .filter(|(_, a1, a2, b1, b2)| {
                        column
                            .iter()
                            .all(|x| a1 <= x && x <= a2 || b1 <= x && x <= b2)
                    })
                    .map(|t| t.0.as_str())
                    .collect::<std::collections::HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let mut correct = vec![""; columns.len()];

        while correct.iter().any(|&x| x == "") {
            for i in 0..valid_interpretations.len() {
                if valid_interpretations[i].len() == 1 {
                    let val = *valid_interpretations[i].iter().nth(0).unwrap();
                    correct[i] = val;

                    for x in valid_interpretations.iter_mut() {
                        x.remove(val);
                    }
                }
            }
        }

        format!(
            "{}",
            correct
                .iter()
                .enumerate()
                .filter(|(_, c)| c.starts_with("departure "))
                .map(|(i, _)| my_ticket[i] as i64)
                .product::<i64>()
        )
    }
}
