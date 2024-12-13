use anyhow::Result;
use helpers::read_input_file;
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
        for machine in machines {
            let xp = machine.prize.x as f64;
            let yp = machine.prize.y as f64;
            let xa = machine.button_a.x as f64;
            let ya = machine.button_a.y as f64;
            let xb = machine.button_b.x as f64;
            let yb = machine.button_b.y as f64;
            let x = 3.0 * (xp * yb - xb * yp) / (xa * yb - xb * ya) / 3.0;
            let y = 3.0 * (xa * yp - xp * ya) / (xa * yb - xb * ya) / 3.0;
            if x.fract() == 0.0 && y.fract() == 0.0 {
                let costs = x as u64 * 3 + y as u64;
                //println!("{} {:?}", costs, (x, y));
                total_costs += costs;
            }
        }

        println!("{}", total_costs);
    }

    Ok(())
}

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
