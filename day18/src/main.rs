use anyhow::Result;
use helpers::read_input_file;
use std::{collections::{HashSet, VecDeque}, env::args};

fn parse_coordinates(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect()
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day18", &input_type)?;
    let width = match input_type.as_str() {
        "test" => 7,
        "real" => 71,
        _ => panic!("Invalid input type"),
    };
    let len = match input_type.as_str() {
        "test" => 12,
        "real" => 1024,
        _ => panic!("Invalid input type"),
    };
    
    let coordinates = parse_coordinates(&contents);
    println!("Parsed {} coordinates", coordinates.len());

    let coordinates_set: HashSet<(usize, usize)> = coordinates[..len].iter().cloned().collect();
    let result = shortest_path(width, width, &coordinates_set);
    println!("Shortest path: {:?}", result);

    for i in len..coordinates.len() {
        let coordinates_set: HashSet<(usize, usize)> = coordinates[..i].iter().cloned().collect();
        let shortest_path = shortest_path(width, width, &coordinates_set);
        if shortest_path.is_none() {
            println!("No path found after {:?}", coordinates[i - 1]);
            break;
        }
    }

    Ok(())
} 

fn shortest_path(
    width: usize,
    height: usize,
    blocked: &HashSet<(usize, usize)>
) -> Option<usize> {
    let start = (0, 0);
    let goal = (height - 1, width - 1);

    if blocked.contains(&start) || blocked.contains(&goal) {
        return None;
    }

    // Directions: up, down, left, right
    let directions = [(0,1), (0,usize::MAX), (1,0), (usize::MAX,0)];
    // We'll use `usize::MAX` to represent subtracting 1:
    // When adding `usize::MAX` to a usize in Rust, it will wrap around,
    // effectively doing `x - 1` when `x > 0`. We'll just have to be careful.

    let mut visited = vec![vec![false; width]; height];
    visited[0][0] = true;

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0)); // (row, col, distance)

    while let Some((r, c, dist)) = queue.pop_front() {
        if (r, c) == goal {
            return Some(dist);
        }

        for &(dr, dc) in &directions {
            // Compute next position
            let nr = if dr == usize::MAX { r.wrapping_sub(1) } else { r + dr };
            let nc = if dc == usize::MAX { c.wrapping_sub(1) } else { c + dc };

            // Check boundaries and if cell is not blocked or visited
            if nr < height && nc < width && !blocked.contains(&(nr, nc)) && !visited[nr][nc] {
                visited[nr][nc] = true;
                queue.push_back((nr, nc, dist + 1));
            }
        }
    }

    // If we reach here, there's no path
    None
}