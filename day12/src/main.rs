use std::{collections::{HashMap, HashSet}, env::args};

use anyhow::Result;
use helpers::read_input_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Horizontal,
    Vertical,
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day12", &input_type)?;
    let contents: Vec<Vec<u8>> = contents.lines().map(|line| line.as_bytes().to_vec()).collect();

    let mut plots: Vec<(u8, HashSet<Position>)> = Vec::new();

    let directions = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
    ];

    for (y, row) in contents.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let mut positions = HashSet::new();
            positions.insert(Position { x: x as i32, y: y as i32 });
            plots.push((c, positions));
        }
    }

    // merge adjacent positions with the same c    
    let mut merged_plots: Vec<(u8, HashSet<Position>)> = Vec::new();
    let mut used_positions: HashSet<Position> = HashSet::new();

    for (i, (c, positions)) in plots.iter().enumerate() {
        if positions.iter().any(|pos| used_positions.contains(pos)) {
            continue;
        }

        let mut merged_positions = positions.clone();
        let mut to_check: Vec<Position> = positions.iter().cloned().collect();

        while let Some(pos) = to_check.pop() {
            for (dx, dy) in directions {
                let new_pos = Position { x: pos.x + dx, y: pos.y + dy };
                
                // Find if there's an adjacent plot with the same character
                if let Some((_, other_positions)) = plots.iter().find(|(other_c, other_positions)| 
                    *other_c == *c && 
                    other_positions.contains(&new_pos) &&
                    !merged_positions.contains(&new_pos)
                ) {
                    merged_positions.insert(new_pos);
                    to_check.push(new_pos);
                }
            }
        }

        used_positions.extend(merged_positions.iter());
        merged_plots.push((*c, merged_positions));
    }

    plots = merged_plots;

    let mut fence = 0;
    for plot in &plots {
        let mut perimeter = 0;
        for pos in &plot.1 {
            for (dx, dy) in directions {
                let new_pos = Position { x: pos.x + dx, y: pos.y + dy };
                if !plot.1.contains(&new_pos) {
                    perimeter += 1;
                }
            }
        }

        println!("{} {} {} {}", plot.0 as char, plot.1.len(), perimeter, plot.1.len() * perimeter);

        fence += plot.1.len() * perimeter;
    }

    let directions = [
        (0, -1, Direction::Horizontal),
        (0, 1, Direction::Horizontal),
        (-1, 0, Direction::Vertical),
        (1, 0, Direction::Vertical),
    ];
    let mut fence = 0;
    for (c, positions) in &plots {
        let mut visited: HashSet<(Position, Direction)> = HashSet::new();

        let mut sides = 0;
        for pos in positions {
            for (dx, dy, direction) in directions {
                let new_pos = Position { x: pos.x + dx, y: pos.y + dy };
                if !positions.contains(&new_pos) && !visited.contains(&(new_pos, direction)) {
                    visited.insert((new_pos, direction));
                    sides += 1;
                    match direction {
                        Direction::Horizontal => {
                            let mut new_x = new_pos.x + 1;
                            while new_x < contents[0].len() as i32 && !positions.contains(&Position { x: new_x, y: pos.y }) {
                                visited.insert((Position { x: new_x, y: pos.y }, direction));
                                new_x += 1;
                            }
                            let mut new_x = new_pos.x - 1;
                            while new_x >= 0 && !positions.contains(&Position { x: new_x, y: pos.y }) {
                                visited.insert((Position { x: new_x, y: pos.y }, direction));
                                new_x -= 1;
                            }
                        }
                        Direction::Vertical => {
                            let mut new_y = new_pos.y + 1;
                            while new_y < contents.len() as i32 && !positions.contains(&Position { x: pos.x, y: new_y }) {
                                visited.insert((Position { x: pos.x, y: new_y }, direction));
                                new_y += 1;
                            }
                            let mut new_y = new_pos.y - 1;
                            while new_y >= 0 && !positions.contains(&Position { x: pos.x, y: new_y }) {
                                visited.insert((Position { x: pos.x, y: new_y }, direction));
                                new_y -= 1;
                            }
                        }
                    }
                }
            }
        }

        fence += visited.len() * sides;
    }

    println!("{}", fence);

    Ok(())
}
