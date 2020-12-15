pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        13
    }

    fn main1(input: &String) -> String {
        let (ts, rest): (i32, String) = codegen_stuff::scan!("{}\n{}\n", input);
        format!(
            "{}",
            rest.split(",")
                .map(|s| s.parse::<i32>().ok())
                .flat_map(|x| x)
                .map(|n| (n, (ts / n + (ts % n != 0) as i32) * n - ts))
                .min_by_key(|p| p.1)
                .map(|(n, w)| n * w)
                .unwrap()
        )
    }

    fn main2(input: &String) -> String {
        let (_, rest): (i32, String) = codegen_stuff::scan!("{}\n{}\n", input);

        let mut k = rest
            .split(",")
            .enumerate()
            .map(|(i, s)| {
                s.parse::<i64>()
                    .ok()
                    .map(|n| (n, (((n - i as i64) % n) + n) % n))
            })
            .flat_map(|x| x)
            .collect::<Vec<_>>();

        k.sort_by_key(|p| -p.0);

        format!(
            "{}",
            k[1..]
                .iter()
                .fold(k[0], |(p, mut res), &(n, a)| {
                    while res % n != a {
                        res += p;
                    }
                    (p * n, res)
                })
                .1
        )
    }
}
