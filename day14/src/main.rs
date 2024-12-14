use anyhow::Result;
use helpers::read_input_file;
use regex::Regex;
use std::{collections::HashMap, env::args};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Size {
    width: u16,
    height: u16,
}

fn parse_input(contents: &str) -> Vec<(Position, Velocity)> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    contents
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|cap| {
                let pos = Position {
                    x: cap[1].parse().unwrap(),
                    y: cap[2].parse().unwrap(),
                };
                let vel = Velocity {
                    x: cap[3].parse().unwrap(),
                    y: cap[4].parse().unwrap(),
                };
                (pos, vel)
            })
        })
        .collect()
}

fn calc_position(
    starting_position: Position,
    velocity: Velocity,
    time: u64,
    size: Size,
) -> Position {
    // Calculate total displacement
    let total_x = starting_position.x as i64 + (velocity.x as i64 * time as i64);
    let total_y = starting_position.y as i64 + (velocity.y as i64 * time as i64);

    // Handle wrapping using modulo
    // We add size before taking modulo to handle negative numbers correctly
    let wrapped_x = ((total_x % size.width as i64 + size.width as i64) % size.width as i64) as u16;
    let wrapped_y =
        ((total_y % size.height as i64 + size.height as i64) % size.height as i64) as u16;

    Position {
        x: wrapped_x,
        y: wrapped_y,
    }
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day14", &input_type)?;

    let robots = parse_input(&contents);
    let size = match input_type.as_str() {
        "test" => Size {
            width: 11,
            height: 7,
        },
        "real" => Size {
            width: 101,
            height: 103,
        },
        _ => unreachable!(),
    };

    part1(&robots, size);
    part2(robots, size);

    Ok(())
}

fn part2(robots: Vec<(Position, Velocity)>, size: Size) {
    let mut positions;
    for t in (65u64..).step_by(103) {
        positions = Vec::new();
        let mut robots_per_position = HashMap::new();
        for robot in &robots {
            let position = calc_position(robot.0, robot.1, t, size);
            positions.push(position);
            robots_per_position
                .entry(position)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        println!("Time: {}", t);
        for y in 0..size.height {
            for x in 0..size.width {
                let count = positions.iter().filter(|p| p.x == x && p.y == y).count();
                print!(
                    "{}",
                    match count {
                        0 => ' ',
                        _ => count.to_string().chars().next().unwrap(),
                    }
                );
            }
            println!();
        }

        // Wait for key press
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

fn part1(robots: &Vec<(Position, Velocity)>, size: Size) {
    let mut robots_per_position = HashMap::new();
    for robot in robots {
        let position = calc_position(robot.0, robot.1, 100, size);
        robots_per_position
            .entry(position)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    // Get the middle point
    let mid_x = size.width / 2;
    let mid_y = size.height / 2;

    // Create a vector to store the count of robots in each quadrant (0 = top-left, 1 = top-right, 2 = bottom-left, 3 = bottom-right)
    let mut quadrant_counts = [0; 4];

    // Count robots in each quadrant, excluding the middle lines
    for (&pos, &count) in &robots_per_position {
        if pos.x == mid_x || pos.y == mid_y {
            continue; // Skip robots on the middle lines
        }

        let quadrant = match (pos.x > mid_x, pos.y > mid_y) {
            (false, false) => 0, // top-left
            (true, false) => 1,  // top-right
            (false, true) => 2,  // bottom-left
            (true, true) => 3,   // bottom-right
        };

        quadrant_counts[quadrant] += count;
    }

    // Multiply the counts together
    let result = quadrant_counts.iter().product::<i32>();
    println!("Result: {}", result);
}
