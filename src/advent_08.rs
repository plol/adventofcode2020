pub struct Advent;

fn parse_program(input: &String) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| {
            let (inst, arg) = codegen_stuff::scan_strs!("{} {}", line);
            (inst, arg.parse().unwrap())
        })
        .collect()
}

fn run_computer(
    program: &Vec<(&str, i32)>,
    mut handler: impl FnMut((&str, i32), i32) -> i32,
) -> bool {
    let mut pc: i32 = 0;

    let mut seen = std::collections::HashSet::new();

    while (pc as usize) < program.len() && !seen.contains(&pc) {
        seen.insert(pc);
        pc += handler(program[pc as usize], pc);
    }
    (pc as usize) < program.len()
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        8
    }

    fn main1(input: &String) -> String {
        let mut acc = 0;
        run_computer(&parse_program(input), |inst, _| match inst {
            ("jmp", x) => x,
            ("acc", x) => {
                acc += x;
                1
            }
            ("nop", _) => 1,
            x => panic!("Unknown instruction {:?}", x),
        });
        format!("{}", acc)
    }

    fn main2(input: &String) -> String {
        let program = parse_program(input);
        let mut seen3 = std::collections::HashSet::new();

        loop {
            let mut has_inserted = false;
            let mut acc = 0;

            let found_infinite_loop = run_computer(&program, |inst, pc| match inst {
                ("jmp", x) => {
                    if !has_inserted && !seen3.contains(&pc) {
                        has_inserted = true;
                        seen3.insert(pc);
                        1
                    } else {
                        x
                    }
                }
                ("acc", x) => {
                    acc += x;
                    1
                }
                ("nop", x) => {
                    if !has_inserted && !seen3.contains(&pc) {
                        has_inserted = true;
                        seen3.insert(pc);
                        x
                    } else {
                        1
                    }
                }
                x => panic!("Unknown instruction {:?}", x),
            });

            if !found_infinite_loop {
                return format!("{}", acc);
            }
        }
    }
}
