pub struct Advent;

struct Rules<'a> {
    transmutation_table: std::collections::HashMap<&'a str, usize>,
    rules: Vec<Vec<(usize, i32)>>,
}

impl<'a> Rules<'a> {
    fn get(&self, k: usize) -> &Vec<(usize, i32)> {
        &self.rules[k]
    }
    fn iter(&self) -> impl Iterator<Item = (usize, &Vec<(usize, i32)>)> {
        self.rules.iter().enumerate()
    }
}

fn can_find_bag(rules: &Rules, k: usize, to_find: usize, memo: &mut Vec<Option<bool>>) -> bool {
    if k == to_find {
        return true;
    }
    memo[k].unwrap_or_else(|| {
        let res = rules
            .get(k)
            .iter()
            .any(|(k2, v)| *v != 0 && can_find_bag(rules, *k2, to_find, memo));
        memo[k] = Some(res);
        res
    })
}

fn count_contained_bags(rules: &Rules, k: usize) -> i32 {
    rules
        .get(k)
        .iter()
        .map(|(k2, v)| {
            if *v == 0 {
                0
            } else {
                *v + *v * count_contained_bags(rules, *k2)
            }
        })
        .sum()
}

fn transmute<'a>(
    transmutation_table: &mut std::collections::HashMap<&'a str, usize>,
    rules: &mut Vec<Vec<(usize, i32)>>,
    k: &'a str,
) -> usize {
    let len = transmutation_table.len();
    *transmutation_table.entry(&k).or_insert_with(|| {
        rules.push(vec![]);
        len
    })
}

fn parse_rules(input: &String) -> Rules {
    //let mut transmutation_table = vec!["no other"];
    let mut transmutation_table = std::collections::HashMap::new();
    let mut rules = vec![];

    transmute(&mut transmutation_table, &mut rules, "no other");

    for line in input.lines() {
        let (container, content) = codegen_stuff::scan_strs!("{} contain {}", line);
        let parsed_container = codegen_stuff::scan_strs!("{} bags", container);
        let transmuted_container =
            transmute(&mut transmutation_table, &mut rules, parsed_container);
        for b in content.split(", ") {
            if b != "no other bags." {
                let (num, spec) = codegen_stuff::scan_strs!("{} {} bag", b);
                let transmuted_spec = transmute(&mut transmutation_table, &mut rules, spec);
                //println!("{:?} {:?}", transmutation_table, transmuted_container);
                rules[transmuted_container].push((transmuted_spec, num.parse().unwrap()));
            }
        }
    }
    Rules {
        transmutation_table,
        rules,
    }
}

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        7
    }

    fn main1(input: &String) -> String {
        let rules = parse_rules(input);
        let mut memo = vec![None; rules.transmutation_table.len()];
        let shiny_gold = *rules.transmutation_table.get("shiny gold").unwrap();
        format!(
            "{}",
            rules
                .iter()
                .filter(|(k, _)| *k != shiny_gold && can_find_bag(&rules, *k, shiny_gold, &mut memo))
                .count()
        )
    }

    fn main2(input: &String) -> String {
        let rules = parse_rules(input);
        //format!("{}", rules.rules.len())
        let shiny_gold = *rules.transmutation_table.get("shiny gold").unwrap();
        format!("{}", count_contained_bags(&rules, shiny_gold))
    }
}
