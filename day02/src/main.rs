use anyhow::Result;
use std::env::args;

use helpers::read_input_file;

fn parse_input(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Rising,
    Falling,
}

fn part1(numbers_collection: &[Vec<i32>], drop_one: bool) {
    let mut number_of_safe = 0;
    for numbers in numbers_collection {
        for drop_ix in -1..numbers.len() as i32 {
            let mut numbers = numbers.clone();
            if drop_ix >= 0 {
                numbers.remove(drop_ix as usize);
            }

            let sign = if numbers[0] - numbers[1] < 0 {
                Direction::Rising
            } else {
                Direction::Falling
            };
            let mut is_safe = true;
            for ix in 1..numbers.len() {
                let distance = numbers[ix] - numbers[ix - 1];
                if distance.abs() < 1
                    || distance.abs() > 3
                    || (distance < 0 && sign == Direction::Rising)
                    || (distance > 0 && sign == Direction::Falling)
                {
                    is_safe = false;
                    break;
                }
            }

            if is_safe {
                number_of_safe += 1;
                break;
            }

            if !drop_one {
                break;
            }
        }
    }
    println!("{}", number_of_safe);
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day02", &input_type)?;
    let numbers_collection = parse_input(&contents);

    part1(&numbers_collection, false);
    part1(&numbers_collection, true);

    Ok(())
}
