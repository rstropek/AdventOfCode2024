use anyhow::Result;
use helpers::{read_input_file, SquareText, DIRECTIONS};
use regex::Regex;
use std::env::args;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day13", &input_type)?;

    let machines_str = contents.split("\n\n").collect::<Vec<&str>>();
    for correction in [0u64, 10000000000000] {
        let machines = parse(&machines_str, correction);

        let mut total_costs = 0;
        for (i, machine) in machines.iter().enumerate() {
            println!("{}", i);
            let mut min_costs = u64::MAX;
            let mut min_moves = (0, 0);
            for b in 0..machine.prize.y / machine.button_b.y {
                let a = (machine.prize.x - b * machine.button_b.x) / machine.button_a.x;
                if machine.prize.x == b * machine.button_b.x + a * machine.button_a.x
                    && machine.prize.y == b * machine.button_b.y + a * machine.button_a.y
                {
                    let costs = a * 3 + b;
                    if costs < min_costs {
                        min_costs = costs;
                        min_moves = (a, b);
                    }
                }
            }
            for a in 0..machine.prize.y / machine.button_a.y {
                let b = (machine.prize.x - a * machine.button_a.x) / machine.button_b.x;
                if machine.prize.x == b * machine.button_b.x + a * machine.button_a.x
                    && machine.prize.y == b * machine.button_b.y + a * machine.button_a.y
                {
                    let costs = a * 3 + b;
                    if costs < min_costs {
                        min_costs = costs;
                        min_moves = (a, b);
                    }
                }
            }
            if min_costs < u64::MAX {
                //println!("{} {:?}", min_costs, min_moves);
                total_costs += min_costs;
            }
        }

        println!("{}", total_costs);
    }

    Ok(())
}

// 35196

fn parse(machines_str: &Vec<&str>, correction: u64) -> Vec<Machine> {
    let number_pattern = Regex::new(r"[+-]\d+").unwrap();
    let price_pattern = Regex::new(r"\d+").unwrap();

    let mut machines = Vec::new();

    for machine in machines_str {
        let lines = machine.lines().collect::<Vec<&str>>();
        let numbers: Vec<&str> = number_pattern
            .find_iter(lines[0])
            .map(|m| m.as_str())
            .collect();

        let button_a = Position {
            x: numbers[0].parse::<u64>().unwrap(),
            y: numbers[1].parse::<u64>().unwrap(),
        };

        let numbers: Vec<&str> = number_pattern
            .find_iter(lines[1])
            .map(|m| m.as_str())
            .collect();

        let button_b = Position {
            x: numbers[0].parse::<u64>().unwrap(),
            y: numbers[1].parse::<u64>().unwrap(),
        };

        let numbers: Vec<&str> = price_pattern
            .find_iter(lines[2])
            .map(|m| m.as_str())
            .collect();

        let prize = Position {
            x: numbers[0].parse::<u64>().unwrap() + correction,
            y: numbers[1].parse::<u64>().unwrap() + correction,
        };

        machines.push(Machine {
            button_a,
            button_b,
            prize,
        });
    }

    machines
}
