use std::{collections::HashSet, env::args};

use anyhow::Result;
use helpers::{read_input_file, SquareText, DIRECTIONS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day12", &input_type)?;
    let contents: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    fn find_all_adjacent(
        contents: &[Vec<u8>],
        id: u8,
        start: Position,
        plot: &mut HashSet<Position>,
        perimeter: &mut HashSet<Position>,
        used_positions: &mut HashSet<Position>,
    ) {
        if contents.is_outside(start.x, start.y)
            || used_positions.contains(&start)
            || contents.get_i32(start.x, start.y) != id
        {
            return;
        }

        plot.insert(start);
        used_positions.insert(start);

        for direction in DIRECTIONS {
            let new_pos = Position {
                x: start.x + direction.0,
                y: start.y + direction.1,
            };

            if contents.is_outside(new_pos.x, new_pos.y)
                || contents.get_i32(new_pos.x, new_pos.y) != id
            {
                perimeter.insert(start);
            }

            find_all_adjacent(contents, id, new_pos, plot, perimeter, used_positions);
        }
    }

    let mut plots: Vec<(u8, HashSet<Position>, HashSet<Position>)> = Vec::new();
    let mut used_positions = HashSet::new();

    for (y, line) in contents.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !used_positions.contains(&Position {
                x: x as i32,
                y: y as i32,
            }) {
                let mut plot = HashSet::new();
                let mut perimeter = HashSet::new();
                find_all_adjacent(
                    &contents,
                    *c,
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    &mut plot,
                    &mut perimeter,
                    &mut used_positions,
                );
                plots.push((*c, plot, perimeter));
            }
        }
    }

    let result = plots.iter().fold(0, |acc, (_, plot, perimeter)| {
        let mut outside = 0;
        perimeter.iter().for_each(|p| {
            for direction in DIRECTIONS {
                let new_pos = Position {
                    x: p.x + direction.0,
                    y: p.y + direction.1,
                };
                if contents.is_outside(new_pos.x, new_pos.y) || !plot.contains(&new_pos) {
                    outside += 1;
                }
            }
        });
        acc + outside * plot.len()
    });

    println!("{}", result);

    let result = plots.iter().fold(0, |acc, (_, plot, perimeter)| {
        fn count_edges(
            perimeter: &HashSet<Position>,
            plot: &HashSet<Position>,
            is_horizontal: bool,
            max_len: usize,
        ) -> i32 {
            let mut edges = 0;

            for i in 0..max_len {
                let mut line_perimeter = perimeter
                    .iter()
                    .filter(|p| {
                        if is_horizontal {
                            p.y == i as i32
                        } else {
                            p.x == i as i32
                        }
                    })
                    .collect::<Vec<_>>();
                if line_perimeter.is_empty() {
                    continue;
                }

                line_perimeter.sort_by(|a, b| {
                    if is_horizontal {
                        a.x.cmp(&b.x)
                    } else {
                        a.y.cmp(&b.y)
                    }
                });

                for offset in [-1i32, 1] {
                    let mut prev: Option<Position> = None;
                    for &p in line_perimeter.iter().filter(|&p| {
                        !plot.contains(&Position {
                            x: p.x + if is_horizontal { 0 } else { offset },
                            y: p.y + if is_horizontal { offset } else { 0 },
                        })
                    }) {
                        match prev {
                            Some(prev) => {
                                let diff = if is_horizontal {
                                    p.x - prev.x
                                } else {
                                    p.y - prev.y
                                };
                                if diff > 1 {
                                    edges += 1;
                                }
                            }
                            None => edges += 1,
                        }
                        prev = Some(*p);
                    }
                }
            }
            edges
        }

        let horizontal_edges = count_edges(perimeter, plot, true, contents.len());
        let vertical_edges = count_edges(perimeter, plot, false, contents[0].len());

        acc + (horizontal_edges + vertical_edges) * plot.len() as i32
    });

    println!("{}", result);

    Ok(())
}
