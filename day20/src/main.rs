use helpers::{read_input_file, SquareText, DIRECTIONS_USIZE};
use std::env::args;
use std::collections::{HashMap, VecDeque, HashSet};

type Graph = HashMap<(usize, usize), Vec<(usize, usize, usize)>>;

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day20", &input_type).unwrap();

    let contents: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    let start = contents.find_byte(b'S');
    let end = contents.find_byte(b'E');

    let graph = build_graph(&contents);
    let original_min = find_shortest_path(&graph, start, end);

    let cheats = get_cheats(&contents, 2);
    for c in &cheats {
        println!("{:?}", c);
    }

    let cheats = vec![((7, 1), (10, 1), 3)];
    let mut saved_time: HashMap<usize, usize> = HashMap::new();
    for (cheat_start, cheat_end, walls) in cheats {
        let mut graph = graph.clone();
        graph.entry(cheat_start).or_insert_with(|| vec![(cheat_end.0, cheat_end.1, walls)]).push((cheat_end.0, cheat_end.1, walls));
        let min = find_shortest_path(&graph, start, end);

        if min < original_min {
            *saved_time.entry(original_min - min).or_insert(0) += 1;
        }
    }

    for (saved, count) in saved_time {
        println!("{}: {}", saved, count);
    }
}

fn build_graph(contents: &[Vec<u8>]) -> Graph {
    let mut graph = HashMap::new();
    
    for y in 0..contents.len() {
        for x in 0..contents[0].len() {
            if contents[y][x] != b'#' {
                let mut neighbors = Vec::new();
                
                for (dx, dy) in DIRECTIONS_USIZE {
                    let next_x = x.wrapping_add(dx);
                    let next_y = y.wrapping_add(dy);
                    
                    if next_y < contents.len() && next_x < contents[0].len() 
                        && contents[next_y][next_x] != b'#' {
                        neighbors.push((next_x, next_y, 1));
                    }
                }
                
                graph.insert((x, y), neighbors);
            }
        }
    }
    
    graph
}

fn get_cheats(contents: &[Vec<u8>], max_walls: usize) -> Vec<((usize, usize), (usize, usize), usize)> {
    let mut cheats = HashSet::new();
    let height = contents.len();
    let width = contents[0].len();

    // For each open space
    for y1 in 0..height {
        for x1 in 0..width {
            if contents[y1][x1] != b'.' {
                continue;
            }

            // Find all (x, y) whose manhattan distance is >= 2 and <= max_walls + 1
            // and where contents[y][x] is not a wall. Store them in cheats with distance = manhattan distance 
            for y2 in 0..height {
                for x2 in 0..width {
                    if contents[y2][x2] != b'.' {
                        continue;
                    }

                    let manhattan_dist = x1.abs_diff(x2) + y1.abs_diff(y2);
                    if manhattan_dist >= 2 && manhattan_dist <= max_walls + 1 {
                        cheats.insert(((x1, y1), (x2, y2), manhattan_dist));
                        cheats.insert(((x2, y2), (x1, y1), manhattan_dist));
                    }
                }
            }

        }
    }

    cheats.into_iter().collect()
}

fn find_shortest_path(graph: &Graph, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((start, 0)); // (position, distance)
    visited.insert(start);
    
    while let Some((current, path_len)) = queue.pop_front() {
        if current == end {
            return path_len;
        }
        
        if let Some(neighbors) = graph.get(&current) {
            for &next in neighbors {
                if !visited.contains(&(next.0, next.1)) {
                    visited.insert((next.0, next.1));
                    queue.push_back(((next.0, next.1), path_len + next.2));
                }
            }
        }
    }
    
    usize::MAX // No path found
}
