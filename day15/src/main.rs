use anyhow::Result;
use helpers::read_input_file;
use std::{collections::HashSet, env::args};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Content {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Movement {
    Left,
    Right,
    Up,
    Down,
}

impl Movement {
    fn get_direction(&self) -> (i32, i32) {
        match self {
            Movement::Left => (-1, 0),
            Movement::Right => (1, 0),
            Movement::Up => (0, -1),
            Movement::Down => (0, 1),
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            Movement::Left => Movement::Right,
            Movement::Right => Movement::Left,
            Movement::Up => Movement::Down,
            Movement::Down => Movement::Up,
        }
    }
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day15", &input_type)?;

    let (map, movements) = contents.split_once("\n\n").unwrap();

    // # = Wall, . = Empty, O = Box, @ = Robot (there can only be one)
    let mut robot_position = None;
    let mut map = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Content::Wall,
                    '.' => Content::Empty,
                    'O' => Content::Box,
                    '@' => {
                        robot_position = Some((x as i32, y as i32));
                        Content::Empty // Robot position is stored separately
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let robot_position = robot_position.expect("Robot position not found in map");

    // < = left, > = right, ^ = up, v = down
    // remove all \n in movements
    let movements = movements
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '<' => Movement::Left,
            '>' => Movement::Right,
            '^' => Movement::Up,
            'v' => Movement::Down,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let mut map_part1 = map.clone();
    part1(&movements, robot_position, &mut map_part1);

    part2(&movements, robot_position, &mut map);

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Content2 {
    Empty,
    Wall,
    BoxLeft,
    BoxRight,
}
fn part2(movements: &[Movement], robot_position: (i32, i32), map: &mut [Vec<Content>]) {
    // Make everything double as wide. # -> ##, . -> .., O -> []
    let mut map2: Vec<Vec<Content2>> = Vec::with_capacity(map.len() * 2);
    for line in map {
        let mut new_line = Vec::with_capacity(line.len() * 2);
        for cell in line {
            match cell {
                Content::Wall => {
                    new_line.push(Content2::Wall);
                    new_line.push(Content2::Wall);
                }
                Content::Empty => {
                    new_line.push(Content2::Empty);
                    new_line.push(Content2::Empty);
                }
                Content::Box => {
                    new_line.push(Content2::BoxLeft);
                    new_line.push(Content2::BoxRight);
                }
            }
        }
        map2.push(new_line);
    }

    let mut map = map2;
    let mut robot_position = (robot_position.0 * 2, robot_position.1);

    //print_map2(&map, robot_position);
    for movement in movements {
        let next_position = (
            robot_position.0 + movement.get_direction().0,
            robot_position.1 + movement.get_direction().1,
        );

        match map[next_position.1 as usize][next_position.0 as usize] {
            Content2::Wall => continue,
            Content2::Empty => robot_position = next_position,
            Content2::BoxLeft | Content2::BoxRight => {
                let mut boxes_to_move = HashSet::new();
                match map[next_position.1 as usize][next_position.0 as usize] {
                    Content2::BoxLeft => {
                        boxes_to_move.insert((
                            next_position.0,
                            next_position.1,
                            map[next_position.1 as usize][next_position.0 as usize],
                        ));
                        boxes_to_move.insert((
                            next_position.0 + 1,
                            next_position.1,
                            map[next_position.1 as usize][(next_position.0 + 1) as usize],
                        ));
                    }
                    Content2::BoxRight => {
                        boxes_to_move.insert((
                            next_position.0,
                            next_position.1,
                            map[next_position.1 as usize][next_position.0 as usize],
                        ));
                        boxes_to_move.insert((
                            next_position.0 - 1,
                            next_position.1,
                            map[next_position.1 as usize][(next_position.0 - 1) as usize],
                        ));
                    }
                    _ => unreachable!(),
                };
                let mut all_empty;
                let mut wall;
                loop {
                    all_empty = true;
                    wall = false;

                    let mut new_boxes_to_move = HashSet::new();
                    for &box_to_move in &boxes_to_move {
                        let next_position = (
                            box_to_move.0 + movement.get_direction().0,
                            box_to_move.1 + movement.get_direction().1,
                        );
                        if boxes_to_move.contains(&(
                            next_position.0,
                            next_position.1,
                            map[next_position.1 as usize][next_position.0 as usize],
                        )) {
                            continue;
                        }

                        match map[next_position.1 as usize][next_position.0 as usize] {
                            Content2::Wall => wall = true,
                            Content2::Empty => continue,
                            Content2::BoxLeft => {
                                new_boxes_to_move.insert((
                                    next_position.0,
                                    next_position.1,
                                    map[next_position.1 as usize][next_position.0 as usize],
                                ));
                                new_boxes_to_move.insert((
                                    next_position.0 + 1,
                                    next_position.1,
                                    map[next_position.1 as usize][(next_position.0 + 1) as usize],
                                ));
                                all_empty = false;
                            }
                            Content2::BoxRight => {
                                new_boxes_to_move.insert((
                                    next_position.0,
                                    next_position.1,
                                    map[next_position.1 as usize][next_position.0 as usize],
                                ));
                                new_boxes_to_move.insert((
                                    next_position.0 - 1,
                                    next_position.1,
                                    map[next_position.1 as usize][(next_position.0 - 1) as usize],
                                ));
                                all_empty = false;
                            }
                        }
                    }

                    boxes_to_move.extend(new_boxes_to_move);

                    if all_empty || wall {
                        break;
                    }
                }

                if all_empty && !wall {
                    loop {
                        // Find a box to move to an empty position
                        let box_to_move = *boxes_to_move
                            .iter()
                            .find(|&b| {
                                map[(b.1 + movement.get_direction().1) as usize]
                                    [(b.0 + movement.get_direction().0) as usize]
                                    == Content2::Empty
                            })
                            .unwrap();
                        map[box_to_move.1 as usize][box_to_move.0 as usize] = Content2::Empty;
                        map[(box_to_move.1 + movement.get_direction().1) as usize]
                            [(box_to_move.0 + movement.get_direction().0) as usize] = box_to_move.2;
                        //print_map2(&map, robot_position);
                        boxes_to_move.remove(&box_to_move);

                        if boxes_to_move.is_empty() {
                            break;
                        }
                    }

                    robot_position = next_position;
                }
            }
        }

        //print_map2(&map, robot_position);
    }


    let mut gps = 0;
    for (iy, line) in map.iter().enumerate() {
        for (ix, &cell) in line.iter().enumerate() {
            if cell == Content2::BoxLeft {
                gps += iy * 100 + ix;
            }
        }
    }

    println!("GPS: {}", gps);
}

fn part1(movements: &[Movement], mut robot_position: (i32, i32), map: &mut [Vec<Content>]) {
    //print_map(&map, robot_position);
    for movement in movements {
        let next_position = (
            robot_position.0 + movement.get_direction().0,
            robot_position.1 + movement.get_direction().1,
        );

        match map[next_position.1 as usize][next_position.0 as usize] {
            Content::Wall => continue,
            Content::Box => {
                // Find first location in the same direction that is empty
                let mut next_empty = next_position;
                while map[next_empty.1 as usize][next_empty.0 as usize] == Content::Box {
                    next_empty = (
                        next_empty.0 + movement.get_direction().0,
                        next_empty.1 + movement.get_direction().1,
                    );
                }

                if map[next_empty.1 as usize][next_empty.0 as usize] == Content::Empty {
                    // move all boxes to the new position
                    while next_empty != next_position {
                        let before_empty = (
                            next_empty.0 + movement.get_opposite().get_direction().0,
                            next_empty.1 + movement.get_opposite().get_direction().1,
                        );
                        map[next_empty.1 as usize][next_empty.0 as usize] = Content::Box;
                        map[before_empty.1 as usize][before_empty.0 as usize] = Content::Empty;
                        next_empty = before_empty;
                    }
                    robot_position = next_position;
                }
            }
            Content::Empty => robot_position = next_position,
        }

        //print_map(&map, robot_position);
    }

    let mut gps = 0;
    for (iy, line) in map.iter().enumerate() {
        for (ix, &cell) in line.iter().enumerate() {
            if cell == Content::Box {
                gps += iy * 100 + ix;
            }
        }
    }

    println!("GPS: {}", gps);
}

fn _print_map(map: &[Vec<Content>], robot_position: (i32, i32)) {
    for (iy, line) in map.iter().enumerate() {
        for (ix, &cell) in line.iter().enumerate() {
            if (ix as i32, iy as i32) == robot_position {
                print!("@");
            } else {
                print!(
                    "{}",
                    match cell {
                        Content::Wall => '#',
                        Content::Empty => '.',
                        Content::Box => 'O',
                    }
                );
            }
        }
        println!();
    }
}

fn _print_map2(map: &[Vec<Content2>], robot_position: (i32, i32)) {
    for (iy, line) in map.iter().enumerate() {
        for (ix, &cell) in line.iter().enumerate() {
            if (ix as i32, iy as i32) == robot_position {
                print!("@");
            } else {
                print!(
                    "{}",
                    match cell {
                        Content2::Wall => "#",
                        Content2::Empty => ".",
                        Content2::BoxLeft => "[",
                        Content2::BoxRight => "]",
                    }
                );
            }
        }
        println!();
    }
}
