use helpers::read_input_file;
use std::{collections::HashSet, env::args};

#[derive(Debug)]
struct Data {
    towels: HashSet<String>,
    designs: Vec<String>,
}

impl Data {
    fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split("\n\n").collect();

        let towels = parts[0].split(", ").map(|s| s.trim().to_string()).collect();

        let designs = parts[1].lines().map(|s| s.trim().to_string()).collect();

        Data { towels, designs }
    }
}

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day19", &input_type).unwrap();

    let data = Data::parse(&contents);

    let result = data
        .designs
        .iter()
        .filter(|design| check(design, &data.towels))
        .count();
    println!("Result: {}", result);

    let count = data
        .designs
        .iter()
        .map(|design| count(design, &data.towels))
        .sum::<u64>();

    println!("Count: {}", count);
}

fn check(design: &str, towels: &HashSet<String>) -> bool {
    if towels.contains(design) {
        return true;
    }

    for towel in towels {
        if design.starts_with(towel) && check(&design[towel.len()..], towels) {
            return true;
        }
    }

    false
}

fn count(design: &str, towels: &HashSet<String>) -> u64 {
    let n = design.len();
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        for towel in towels {
            let wlen = towel.len();
            if i >= wlen && &design[i - wlen..i] == towel {
                dp[i] += dp[i - wlen];
            }
        }
    }

    dp[n]
}
