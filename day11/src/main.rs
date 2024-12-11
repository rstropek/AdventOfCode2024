use std::{collections::HashMap, env::args};

use anyhow::Result;
use helpers::read_input_file;

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day11", &input_type)?;

    let stones = contents
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", solve(&stones, 25));
    println!("{}", solve(&stones, 75));

    Ok(())
}

fn solve(stones: &[u64], iterations: usize) -> usize {
    let mut stone_counts: HashMap<u64, usize> = stones
        .iter()
        .fold(HashMap::new(), |mut acc, &stone| {
            *acc.entry(stone).or_insert(0) += 1;
            acc
        });

    for _ in 0..iterations {
        stone_counts = stone_counts
            .into_iter()
            .flat_map(|(stone, count)| match stone {
                0 => vec![(1, count)],
                s => {
                    let number_of_digits = (s as f64).log10().floor() as u32 + 1;
                    if number_of_digits % 2 == 0 {
                        let power = 10u64.pow(number_of_digits / 2);
                        vec![(s / power, count), (s % power, count)]
                    } else {
                        vec![(s * 2024, count)]
                    }
                }
            })
            .fold(HashMap::new(), |mut acc, (stone, count)| {
                *acc.entry(stone).or_insert(0) += count;
                acc
            });
    }

    stone_counts.values().sum()
}
