pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        14
    }

    fn main1(input: &String) -> String {
        let mut memory = Vec::<u64>::new();
        let mut positive_mask: u64 = 0;
        let mut negaitve_mask: u64 = !0;

        for line in input.lines() {
            if line.starts_with("mask = ") {
                positive_mask = 0;
                negaitve_mask = !0;
                let mask_str = &line["mask = ".len()..];
                for i in 0..mask_str.len() {
                    match mask_str.as_bytes()[mask_str.len() - 1 - i] {
                        b'1' => positive_mask |= 1 << i,
                        b'0' => negaitve_mask &= !(1 << i),
                        _ => {}
                    }
                }
            } else {
                let (addr, val): (usize, u64) = codegen_stuff::scan!("mem[{}] = {}", line);
                if addr >= memory.len() {
                    memory.extend((memory.len()..addr + 1).map(|_| 0));
                }
                memory[addr] = val & negaitve_mask | positive_mask;
            }
        }
        format!("{}", memory.iter().sum::<u64>())
    }

    fn main2(input: &String) -> String {
        let mut memory = std::collections::HashMap::new();
        let mut positive_mask: u64 = 0;
        let mut negaitve_mask: u64 = !0;
        let mut wobbly_bits = vec![];

        for line in input.lines() {
            if line.starts_with("mask = ") {
                positive_mask = 0;
                negaitve_mask = !0;
                wobbly_bits.clear();
                let mask_str = &line["mask = ".len()..];
                for i in 0..mask_str.len() {
                    match mask_str.as_bytes()[mask_str.len() - 1 - i] {
                        b'1' => positive_mask |= 1 << i,
                        b'X' => {
                            negaitve_mask &= !(1 << i);
                            wobbly_bits.push(i)
                        }
                        _ => {}
                    }
                }
                wobbly_bits.reverse();
            } else {
                let (addr, val): (u64, u64) = codegen_stuff::scan!("mem[{}] = {}", line);
                wow(&wobbly_bits, 0, 0, &mut |wobbly_mask| {
                    let wobbled_addr = (addr & negaitve_mask | positive_mask) | wobbly_mask;
                    memory.insert(wobbled_addr, val);
                })
            }
        }
        format!("{}", memory.values().sum::<u64>())
    }
}

fn wow(wobbly_bits: &Vec<usize>, i: usize, r: u64, cb: &mut impl FnMut(u64)) {
    if i < wobbly_bits.len() {
        wow(wobbly_bits, i + 1, r, cb);
        wow(wobbly_bits, i + 1, r | (1 << wobbly_bits[i]), cb);
    } else {
        cb(r);
    }
}
