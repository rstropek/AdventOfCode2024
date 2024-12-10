use std::{collections::HashSet, env::args};

use anyhow::Result;
use helpers::read_input_file;

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day10", &input_type)?;

    let lines = contents.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut trailheads = Vec::<(usize, usize)>::new();

    for (y, &row) in lines.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'0' {
                trailheads.push((x, y));
            }
        }
    }

    let mut paths = 0;
    let mut score = 0;
    for &trailhead in &trailheads {
        let mut targets = HashSet::new();
        paths += count_all_paths_to_9(trailhead, &lines, &mut targets);
        score += targets.len();
    }
    println!("{}", score);
    println!("{}", paths);

    Ok(())
}

fn count_all_paths_to_9(start: (usize, usize), lines: &[&[u8]], targets: &mut HashSet<(usize, usize)>) -> i32 {
    if lines[start.1][start.0] == b'9' {
        targets.insert(start);
        return 1
    }

    let mut paths = 0;
    let current = lines[start.1][start.0];
    if start.0 > 0 && lines[start.1][start.0 - 1] == current + 1 {
        paths += count_all_paths_to_9((start.0 - 1, start.1), lines, targets);
    }

    if start.0 < lines[0].len() - 1 && lines[start.1][start.0 + 1] == current + 1 {
        paths += count_all_paths_to_9((start.0 + 1, start.1), lines, targets);
    }

    if start.1 > 0 && lines[start.1 - 1][start.0] == current + 1 {
        paths += count_all_paths_to_9((start.0, start.1 - 1), lines, targets);
    }

    if start.1 < lines.len() - 1 && lines[start.1 + 1][start.0] == current + 1 {
        paths += count_all_paths_to_9((start.0, start.1 + 1), lines, targets);
    }

    paths
}
