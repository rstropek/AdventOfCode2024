use helpers::read_input_file;
use std::{env::args, ops::Index};

use anyhow::Result;

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day05", &input_type)?;

    let sections: Vec<&str> = contents.split("\n\n").collect();

    let rules: Vec<(u32, u32)> = sections[0]
        .lines()
        .map(|line| {
            let rule_parts: Vec<u32> = line.split("|").map(|part| part.parse().unwrap()).collect();
            (rule_parts[0], rule_parts[1])
        })
        .collect();

    let updates: Vec<Vec<u32>> = sections[1]
        .lines()
        .map(|line| line.split(",").map(|part| part.parse().unwrap()).collect())
        .collect();

    let mut middle_sum = 0;
    let mut middle_sum_2 = 0;
    for mut update in updates {
        if is_valid(&rules, &update) {
            middle_sum += update[update.len() / 2];
        } else {
            loop {
                for (first, second) in rules.iter() {
                    let first_index = update.iter().position(|&x| x == *first);
                    let second_index = update.iter().position(|&x| x == *second);
                    if first_index.is_some() && second_index.is_some() && first_index > second_index
                    {
                        let elem = update.remove(first_index.unwrap());
                        update.insert(second_index.unwrap(), elem);
                    }
                }

                if is_valid(&rules, &update) {
                    break;
                }
            }

            middle_sum_2 += update[update.len() / 2];
        }
    }

    println!("Middle sum: {}", middle_sum);
    println!("Middle sum 2: {}", middle_sum_2);

    Ok(())
}

fn is_valid(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    let mut is_valid = true;
    for (first, second) in rules.iter() {
        // Get index of first in update
        let first_index = update.iter().position(|&x| x == *first);
        let second_index = update.iter().position(|&x| x == *second);
        if first_index.is_some() && second_index.is_some() && first_index > second_index {
            is_valid = false;
            break;
        }
    }
    is_valid
}
