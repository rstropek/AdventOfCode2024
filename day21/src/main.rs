use helpers::read_input_file;
use std::{collections::HashMap, env::args};

const NUM_KEYPAD: [(char, (i8, i8)); 11] = [
    ('7', (0, 0)),
    ('8', (1, 0)),
    ('9', (2, 0)),
    ('4', (0, 1)),
    ('5', (1, 1)),
    ('6', (2, 1)),
    ('1', (0, 2)),
    ('2', (1, 2)),
    ('3', (2, 2)),
    ('0', (1, 3)),
    ('A', (2, 3)),
];
const NUM_LAVA: (i8, i8) = (0, 3);

const DIRECTION_PAD: [(char, (i8, i8)); 5] = [('^', (1, 0)), ('A', (2, 0)), ('<', (0, 1)), ('v', (1, 1)), ('>', (2, 1))];
const DIRECTION_LAVA: (i8, i8) = (0, 0);

const DIRECTION_KEYS: [((i8, i8), char); 4] = [((1, 0), '>'), ((0, 1), 'v'), ((-1, 0), '<'), ((0, -1), '^')];

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day21", &input_type).unwrap();
    let contents = contents.lines().collect::<Vec<_>>();

    let num_pad = HashMap::from(NUM_KEYPAD);
    let direction_pad = HashMap::from(DIRECTION_PAD);
    let direction_keys = HashMap::from(DIRECTION_KEYS);

    /*
    let start = *num_pad.get(&b'7').unwrap();
    let end = *num_pad.get(&b'A').unwrap();
    let path = get_path2(start, end, NUM_LAVA, &direction_keys);
    for p in path {
        println!("{}", String::from_utf8(p.clone()).unwrap());
    }

    let start = *direction_pad.get(&b'A').unwrap();
    let end = *direction_pad.get(&b'>').unwrap();
    let path = get_path2(start, end, DIRECTION_LAVA, &direction_keys);
    for p in path {
        println!("{}", String::from_utf8(p.clone()).unwrap());
    }
    */

    let mut sum = 0;
    for line in contents {
        let mut paths = get_path_for_sequence(line, &num_pad, NUM_LAVA, &direction_keys);

        for _ in 0..2 {
            let mut new_paths = vec![];
            for p in &paths {
                let next_paths = get_path_for_sequence(p, &direction_pad, DIRECTION_LAVA, &direction_keys);
                new_paths.extend(next_paths);
            }

            paths = new_paths;
        }

        let shortest_path = paths.iter().min_by_key(|p| p.len()).unwrap();

        let num = line[..line.len() - 1].parse::<usize>().unwrap();
        println!("{} {}", num, shortest_path.len());
        sum += num * shortest_path.len();
    }

    println!("{}", sum);
}

fn get_path_for_sequence(sequence: &str, key_pad: &HashMap<char, (i8, i8)>, lava: (i8, i8), keys: &HashMap<(i8, i8), char>) -> Vec<String> {
    let mut pos = *key_pad.get(&'A').unwrap();
    let mut results: Vec<String> = vec![String::new()];
    for ix in 0..sequence.len() {
        let end = *key_pad.get(&(sequence.as_bytes()[ix] as char)).unwrap();
        let sub_path = get_path2(pos, end, lava, &keys);
        let mut new_results = vec![];
        for p in results {
            for sp in &sub_path {
                let mut new_p = p.clone();
                new_p.push_str(sp);
                new_results.push(new_p);
            }
        }
        results = new_results;
        pos = end;
    }
    results
}

fn get_path(start: (i8, i8), end: (i8, i8), lava: (i8, i8), keys: &HashMap<(i8, i8), u8>) -> Vec<u8> {
    let mut path = vec![];
    let mut pos = start;

    while pos != end {
        let mut next_pos;

        let mut vertical_first = true;
        loop {
            next_pos = pos;
            if vertical_first {
                if pos.1 != end.1 {
                    next_pos.1 = if pos.1 < end.1 { pos.1 + 1 } else { pos.1 - 1 };
                } else if pos.0 != end.0 {
                    next_pos.0 = if pos.0 < end.0 { pos.0 + 1 } else { pos.0 - 1 };
                }
            } else {
                if pos.0 != end.0 {
                    next_pos.0 = if pos.0 < end.0 { pos.0 + 1 } else { pos.0 - 1 };
                } else if pos.1 != end.1 {
                    next_pos.1 = if pos.1 < end.1 { pos.1 + 1 } else { pos.1 - 1 };
                }
            }

            if lava == next_pos {
                vertical_first = !vertical_first;
            } else {
                break;
            }
        }

        path.push(*keys.get(&(next_pos.0 - pos.0, next_pos.1 - pos.1)).unwrap());
        pos = next_pos;
    }

    path.push(b'A');
    path
}

fn get_path2(start: (i8, i8), end: (i8, i8), lava: (i8, i8), keys: &HashMap<(i8, i8), char>) -> Vec<String> {
    use std::collections::{HashSet, VecDeque};

    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Each queue entry contains: (position, path_so_far)
    queue.push_back((start, String::new()));
    visited.insert(start);

    let mut min_length = None;

    while let Some((pos, path)) = queue.pop_front() {
        // If we found a path and it's longer than a previously found path, skip it
        if let Some(len) = min_length {
            if path.len() > len {
                continue;
            }
        }

        // If we reached the end, save the path
        if pos == end {
            if min_length.is_none() {
                min_length = Some(path.len());
            }
            let mut final_path = path.clone();
            final_path.push('A');
            paths.push(final_path);
            continue;
        }

        // Try all possible moves: right, down, left, up
        for &(dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_pos = (pos.0 + dx, pos.1 + dy);

            // Skip if it's the lava cell or out of bounds
            if next_pos == lava || next_pos.0 < 0 || next_pos.0 > 2 || next_pos.1 < 0 || next_pos.1 > 3 {
                continue;
            }

            // Create the new path
            let mut new_path = path.clone();
            new_path.push(*keys.get(&(dx, dy)).unwrap());

            // Add to queue if this is a shortest path to this position
            if !visited.contains(&next_pos) || min_length.is_none() {
                queue.push_back((next_pos, new_path));
                visited.insert(next_pos);
            }
        }
    }

    paths
}
