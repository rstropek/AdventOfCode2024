use anyhow::Result;
use helpers::read_input_file;
use std::{collections::HashSet, env::args};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Antenna {
    position: (i32, i32),
    frequency: u8,
}

fn is_in_bounds(position: (i32, i32), contents: &[Vec<u8>]) -> bool {
    position.0 >= 0
        && position.0 < contents[0].len() as i32
        && position.1 >= 0
        && position.1 < contents.len() as i32
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day08", &input_type)?;
    let contents: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let mut antennas = Vec::new();
    let mut frequencies = HashSet::new();
    for (y, line) in contents.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != b'.' {
                antennas.push(Antenna {
                    position: (x as i32, y as i32),
                    frequency: c,
                });
                frequencies.insert(c);
            }
        }
    }

    part1(&frequencies, &antennas, &contents);
    part2(&frequencies, &antennas, &contents);
    Ok(())
}

fn part1(frequencies: &HashSet<u8>, antennas: &[Antenna], contents: &[Vec<u8>]) {
    let mut antinodes = HashSet::new();
    for &frequency in frequencies {
        let current_antennas: Vec<&Antenna> = antennas
            .iter()
            .filter(|antenna| antenna.frequency == frequency)
            .collect();
        for (i, &antenna) in current_antennas.iter().enumerate() {
            for &other_antenna in current_antennas.iter().skip(i + 1) {
                for (a1, a2) in [(antenna, other_antenna), (other_antenna, antenna)] {
                    let dx = (a2.position.0 - a1.position.0).abs();
                    let dy = (a2.position.1 - a1.position.1).abs();
                    let direction_x = (a1.position.0 - a2.position.0).signum();
                    let direction_y = (a1.position.1 - a2.position.1).signum();
                    let antinode_position = (
                        a1.position.0 + dx * direction_x,
                        a1.position.1 + dy * direction_y,
                    );
                    if is_in_bounds(antinode_position, contents) {
                        {
                            antinodes.insert(antinode_position);
                        }
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn part2(frequencies: &HashSet<u8>, antennas: &[Antenna], contents: &[Vec<u8>]) {
    let mut antinodes = HashSet::new();
    for &frequency in frequencies {
        let current_antennas: Vec<&Antenna> = antennas
            .iter()
            .filter(|antenna| antenna.frequency == frequency)
            .collect();
        for (i, &antenna) in current_antennas.iter().enumerate() {
            for &other_antenna in current_antennas.iter().skip(i + 1) {
                for (a1, a2) in [(antenna, other_antenna), (other_antenna, antenna)] {
                    let dx = (a2.position.0 - a1.position.0).abs();
                    let dy = (a2.position.1 - a1.position.1).abs();

                    let direction_x = (a1.position.0 - a2.position.0).signum();
                    let direction_y = (a1.position.1 - a2.position.1).signum();
                    antinodes.insert(a1.position);
                    let mut antinode_position = a1.position;
                    loop {
                        antinode_position = (
                            antinode_position.0 + dx * direction_x,
                            antinode_position.1 + dy * direction_y,
                        );
                        if is_in_bounds(antinode_position, contents) {
                            antinodes.insert(antinode_position);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
