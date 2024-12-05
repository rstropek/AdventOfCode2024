use helpers::read_input_file;
use std::env::args;

use anyhow::Result;

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day04", &input_type)?;

    // Convert each line to Vec<u8> containing ASCII bytes
    let content_lines: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    part1(&content_lines);
    part2(&content_lines);
    Ok(())
}

fn part1(content_lines: &Vec<Vec<u8>>) {
    let mut xmas_count = 0;
    for y in 0..content_lines.len() {
        for x in 0..content_lines[y].len() {
            let directions = vec![
                (1i32, 0i32),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ];

            for (dx, dy) in directions {
                let x = x as i32;
                let y = y as i32;
                let end_x = x + dx * 3;
                let end_y = y + dy * 3;
                if end_x >= 0
                    && end_y >= 0
                    && end_x < content_lines[y as usize].len() as i32
                    && end_y < content_lines.len() as i32
                {
                    let pattern = [
                        content_lines[y as usize][x as usize],
                        content_lines[(y + dy) as usize][(x + dx) as usize],
                        content_lines[(y + dy * 2) as usize][(x + dx * 2) as usize],
                        content_lines[(y + dy * 3) as usize][(x + dx * 3) as usize],
                    ];
                    if pattern == [b'X', b'M', b'A', b'S'] {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    println!("XMAS count: {}", xmas_count);
}

fn part2(content_lines: &Vec<Vec<u8>>) {
    let mut mas_count = 0;
    for y in 0..content_lines.len() {
        for x in 0..content_lines[y].len() {
            if x > 0 && y > 0 && x < content_lines[y].len() - 1 && y < content_lines.len() - 1 {
                let pattern = [
                    content_lines[y][x],
                    content_lines[y - 1][x - 1],
                    content_lines[y - 1][x + 1],
                    content_lines[y + 1][x - 1],
                    content_lines[y + 1][x + 1],
                ];
                if pattern[0] == b'A' {
                    if ((pattern[1] == b'M' && pattern[4] == b'S')
                        || (pattern[1] == b'S' && pattern[4] == b'M'))
                        && ((pattern[2] == b'M' && pattern[3] == b'S')
                            || (pattern[2] == b'S' && pattern[3] == b'M'))
                    {
                        mas_count += 1;
                    }
                }
            }
        }
    }

    println!("XMAS count: {}", mas_count);
}
