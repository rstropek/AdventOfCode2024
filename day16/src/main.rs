use std::{cmp::Ordering, collections::{BinaryHeap, HashSet, VecDeque}, env::args};

use anyhow::Result;
use helpers::{read_input_file, DIRECTIONS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
    position: (i32, i32),
    direction: usize,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct State {
    cost: u32,
    position: (i32, i32),
    dir: usize, // 0=N,1=E,2=S,3=W
}

// For the priority queue, we want the smallest cost first
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // reverse because BinaryHeap is max-first
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day16", &input_type)?;

    let maze: Vec<Vec<u8>> = contents.lines().map(|line| line.as_bytes().to_vec()).collect();

    let rows = maze.len();
    let cols = maze[0].len();

    // Find start and end
    let mut reindeer = Reindeer {
        position: (0, 0),
        direction: 1, // east
    };
    let mut end = (0, 0);

    for y in 0..rows {
        for x in 0..cols {
            if maze[y][x] == b'S' {
                reindeer.position = (x as i32, y as i32);
            } else if maze[y][x] == b'E' {
                end = (x as i32, y as i32);
            }
        }
    }


    let (costs, cells) = solve(cols, rows, reindeer.clone(), end, maze.clone());

    println!("Part 1: {:?}", costs);
    println!("Part 2: {:?}", cells);

    Ok(())
}

fn solve(cols: usize, rows: usize, reindeer: Reindeer, end: (i32, i32), maze: Vec<Vec<u8>>) -> (u32, u32) {
    let mut dist = vec![vec![vec![u32::MAX;4];cols];rows];
    
    // Parents: For each (y, x, direction), store a list of predecessors that lead to this state on shortest paths
    let mut parents = vec![vec![vec![Vec::new();4];cols];rows];

    let start_dir = 1 as usize; // east
    dist[reindeer.position.1 as usize][reindeer.position.0 as usize][start_dir as usize] = 0;

    // Dijkstra with priority queue (https://takeuforward.org/data-structure/dijkstras-algorithm-using-priority-queue-g-32/)
    let mut pq = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        position: reindeer.position,
        dir: start_dir
    });

    while let Some(State { cost, position, dir }) = pq.pop() {
        let (x_usize, y_usize) = (position.0 as usize, position.1 as usize);
        if cost > dist[y_usize][x_usize][dir as usize] {
            continue;
        }

        // If we've reached the end with minimal cost, keep going to find all shortest paths.
        // We'll process all shortest paths at the end.

        // Try forward
        {
            let (dx, dy) = DIRECTIONS[dir as usize];
            let nx = position.0 + dx;
            let ny = position.1 + dy;
            if ny >= 0 && ny < rows as i32 && nx >= 0 && nx < cols as i32 {
                let (nx_usize, ny_usize) = (nx as usize, ny as usize);
                if maze[ny_usize][nx_usize] != b'#' {
                    let forward_cost = cost + 1;
                    let old_cost = dist[ny_usize][nx_usize][dir as usize];
                    if forward_cost < old_cost {
                        dist[ny_usize][nx_usize][dir as usize] = forward_cost;
                        parents[ny_usize][nx_usize][dir as usize].clear();
                        parents[ny_usize][nx_usize][dir as usize].push((position.1, position.0, dir));
                        pq.push(State { cost: forward_cost, position: (nx, ny), dir });
                    } else if forward_cost == old_cost {
                        // Another equally good path
                        parents[ny_usize][nx_usize][dir as usize].push((position.1, position.0, dir));
                    }
                }
            }
        }

        // Turn left
        {
            let left_dir = (dir + 3) % 4;
            let (ldx, ldy) = DIRECTIONS[left_dir as usize];
            let lx = position.0 + ldx;
            let ly = position.1 + ldy;
            if ly >= 0 && ly < rows as i32 && lx >= 0 && lx < cols as i32 {
                let (lx_usize, ly_usize) = (lx as usize, ly as usize);
                if maze[ly_usize][lx_usize] != b'#' {
                    let left_cost = cost + 1000 + 1;
                    let old_cost = dist[ly_usize][lx_usize][left_dir as usize];
                    if left_cost < old_cost {
                        dist[ly_usize][lx_usize][left_dir as usize] = left_cost;
                        parents[ly_usize][lx_usize][left_dir as usize].clear();
                        parents[ly_usize][lx_usize][left_dir as usize].push((position.1, position.0, dir));
                        pq.push(State { cost: left_cost, position: (lx, ly), dir: left_dir });
                    } else if left_cost == old_cost {
                        parents[ly_usize][lx_usize][left_dir as usize].push((position.1, position.0, dir));
                    }
                }
            }
        }

        // Turn right
        {
            let right_dir = (dir + 1) % 4;
            let (rdx, rdy) = DIRECTIONS[right_dir as usize];
            let rx = position.0 + rdx;
            let ry = position.1 + rdy;
            if ry >= 0 && ry < rows as i32 && rx >= 0 && rx < cols as i32 {
                let (rx_usize, ry_usize) = (rx as usize, ry as usize);
                if maze[ry_usize][rx_usize] != b'#' {
                    let right_cost = cost + 1000 + 1;
                    let old_cost = dist[ry_usize][rx_usize][right_dir as usize];
                    if right_cost < old_cost {
                        dist[ry_usize][rx_usize][right_dir as usize] = right_cost;
                        parents[ry_usize][rx_usize][right_dir as usize].clear();
                        parents[ry_usize][rx_usize][right_dir as usize].push((position.1, position.0, dir));
                        pq.push(State { cost: right_cost, position: (rx, ry), dir: right_dir });
                    } else if right_cost == old_cost {
                        parents[ry_usize][rx_usize][right_dir as usize].push((position.1, position.0, dir));
                    }
                }
            }
        }
    }

    // Find the minimal cost at the end cell over all directions
    let mut min_cost_to_end = u32::MAX;
    let mut best_dirs = Vec::new();
    for d in 0..4 {
        if dist[end.1 as usize][end.0 as usize][d] < min_cost_to_end {
            min_cost_to_end = dist[end.1 as usize][end.0 as usize][d];
            best_dirs.clear();
            best_dirs.push(d);
        } else if dist[end.1 as usize][end.0 as usize][d] == min_cost_to_end {
            best_dirs.push(d);
        }
    }

    if min_cost_to_end == u32::MAX {
        println!("No path found.");
        return (0, 0);
    }

    let min_cost_to_end = min_cost_to_end;

    // Now we reconstruct all shortest paths. We'll start from (end_x, end_y)
    // and all best directions that achieved min_cost_to_end.
    let mut cells_on_min_paths = HashSet::new();
    let mut queue = VecDeque::new();

    // Start from end states
    for &d in &best_dirs {
        queue.push_back((end.0 as i32, end.1 as i32, d as u8));
    }

    while let Some((cx, cy, cdir)) = queue.pop_front() {
        // Mark this cell as on a shortest path
        cells_on_min_paths.insert((cx, cy));

        // Follow parents
        for &(py, px, pdir) in &parents[cy as usize][cx as usize][cdir as usize] {
            // Add parents to queue if not visited in this reconstruction step
            // We might not need a visited set for reconstruction if we just want union of cells:
            // But let's add a visited set to avoid infinite loops if something went wrong.
            if !cells_on_min_paths.contains(&(px, py)) {
                queue.push_back((px, py, pdir as u8));
            }
        }
    }


    (min_cost_to_end, cells_on_min_paths.len() as u32)
}
