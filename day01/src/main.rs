use std::env::args;
use anyhow::Result;

use helpers::read_input_file;

fn parse_input(contents: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut numbers_a: Vec<i32> = Vec::with_capacity(lines.len());
    let mut numbers_b: Vec<i32> = Vec::with_capacity(lines.len());
    lines.iter()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(a, b)| {
            numbers_a.push(a.parse::<i32>().unwrap());
            numbers_b.push(b.parse::<i32>().unwrap());
        });
    Ok((numbers_a, numbers_b))
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day01", &input_type)?;
    let (mut numbers_a, mut numbers_b) = parse_input(&contents)?;

    // Part 1
    numbers_a.sort();
    numbers_b.sort();
    let result = numbers_a.iter()
        .enumerate()
        .map(|(i, &a)| (numbers_b.get(i).unwrap() - a).abs())
        .sum::<i32>();
    println!("{result}");

    // Part 2
    let result: usize = numbers_a.iter()
        .map(|&a| numbers_b.iter().filter(|&&b| b == a).count() * a as usize)
        .sum();
    println!("{result}");

    Ok(())
}
