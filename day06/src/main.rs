use helpers::read_input_file;
use std::{collections::HashSet, env::args};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Maze {
    map: Vec<Vec<Cell>>,
    visited: HashSet<(i32, i32)>,
    visited_with_direction: HashSet<(i32, i32, Direction)>,
    position: (i32, i32),
    direction: Direction,
}

impl Maze {
    fn go(&mut self) -> (bool, bool) {
        let next_position = self.next_position();

        // Return false if next_position is out of bounds
        if next_position.0 < 0
            || next_position.0 >= self.map.len() as i32
            || next_position.1 < 0
            || next_position.1 >= self.map[0].len() as i32
        {
            return (false, false);
        }

        // If there is something directly in front of you, turn right 90 degrees.
        // Otherwise, take a step forward.
        let next_cell = self.map[next_position.1 as usize][next_position.0 as usize];
        if next_cell == Cell::Blocked {
            self.direction = self.direction.right();
        } else {
            self.position = next_position;
            self.visited.insert(self.position);
            if !self.visited_with_direction.insert((
                self.position.0,
                self.position.1,
                self.direction,
            )) {
                return (false, true);
            }
        }

        (true, false)
    }

    fn next_position(&self) -> (i32, i32) {
        let (x, y) = self.position;
        match self.direction {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day06", &input_type)?;

    let mut position = (0, 0);
    let map = contents
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Blocked,
                    '^' => {
                        position = (x as i32, y as i32);
                        Cell::Empty
                    }
                    _ => panic!("Invalid character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let maze = Maze {
        map: map.clone(),
        visited: HashSet::new(),
        visited_with_direction: HashSet::new(),
        position,
        direction: Direction::North,
    };

    let mut visitations_maze = maze.clone();
    loop {
        let (success, _) = visitations_maze.go();
        if !success {
            break;
        }
    }

    let mut modifications_that_lead_to_loop = 0;
    for (x, y) in visitations_maze.visited.iter() {
        if maze.map[*y as usize][*x as usize] == Cell::Blocked || (*x, *y) == maze.position {
            continue;
        }

        let mut modified_maze = maze.clone();
        modified_maze.map[*y as usize][*x as usize] = Cell::Blocked;
        let mut success: bool;
        let mut loop_detected: bool;
        loop {
            (success, loop_detected) = modified_maze.go();
            if !success || loop_detected {
                break;
            }
        }

        if loop_detected {
            modifications_that_lead_to_loop += 1;
        }
    }

    println!("Visited: {}", visitations_maze.visited.len());
    println!(
        "Modifications that lead to loop: {}",
        modifications_that_lead_to_loop
    );
    Ok(())
}
