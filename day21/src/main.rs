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

type Map = HashMap<((i8, i8), (i8, i8)), Vec<String>>;

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day21", &input_type).unwrap();
    let contents = contents.lines().collect::<Vec<_>>();

    let num_pad = HashMap::from(NUM_KEYPAD);
    let direction_pad = HashMap::from(DIRECTION_PAD);
    let direction_keys = HashMap::from(DIRECTION_KEYS);

    let mut keypad_map = HashMap::new();
    for &v in num_pad.values() {
        for &v2 in num_pad.values() {
            let paths = get_path(v, v2, NUM_LAVA, &direction_keys);
            keypad_map.insert((v, v2), paths);
        }
    }

    let mut direction_map = HashMap::new();
    for &v in direction_pad.values() {
        for &v2 in direction_pad.values() {
            let paths = get_path(v, v2, DIRECTION_LAVA, &direction_keys);
            direction_map.insert((v, v2), paths);
        }
    }

    let mut sum = 0;
    for line in &contents {
        let paths = get_path_for_sequence(line, &num_pad, &keypad_map);
        let min_len = paths.iter().map(|p| recurse::<2>(p, 0, &direction_pad, &direction_map, &mut HashMap::new())).min().unwrap();
        let num = line[..line.len() - 1].parse::<usize>().unwrap();
        sum += num * min_len;
    }
    println!("Part 1: {}", sum);

    let mut sum = 0;
    for line in &contents {
        let paths = get_path_for_sequence(line, &num_pad, &keypad_map);
        let min_len = paths.iter().map(|p| recurse::<25>(p, 0, &direction_pad, &direction_map, &mut HashMap::new())).min().unwrap();
        let num = line[..line.len() - 1].parse::<usize>().unwrap();
        sum += num * min_len;
    }

    println!("Part 2: {}", sum);
}

fn recurse<const R: usize>(sequence: &str, level: usize, pad: &HashMap<char, (i8, i8)>, map: &Map, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if level == R {
        return sequence.len();
    }

    let mut total_len = 0;
    let mut pos = *pad.get(&'A').unwrap();
    for c in sequence.chars() {
        let end = *pad.get(&c).unwrap();
        let sub_path = map.get(&(pos, end)).unwrap();
        let mut lens = Vec::with_capacity(sub_path.len());
        for sp in sub_path {
            if let Some(len) = cache.get(&(sp.clone(), level + 1)) {
                lens.push(*len);
            } else {
                let len = recurse::<R>(sp, level + 1, pad, map, cache);
                cache.insert((sp.clone(), level + 1), len);
                lens.push(len);
            }
        }

        total_len += lens.iter().min().unwrap();
        pos = end;
    }

    total_len
}

fn get_path_for_sequence(sequence: &str, key_pad: &HashMap<char, (i8, i8)>, key_map: &Map) -> Vec<String> {
    let mut pos = *key_pad.get(&'A').unwrap();
    let mut results: Vec<String> = vec![String::new()];
    for ix in 0..sequence.len() {
        let end = *key_pad.get(&(sequence.as_bytes()[ix] as char)).unwrap();
        let sub_path = key_map.get(&(pos, end)).unwrap();
        let mut new_results = vec![];
        for p in results {
            for sp in sub_path {
                let mut new_p = p.clone();
                new_p.push_str(sp);
                new_results.push(new_p);
            }
        }
        let min_len = new_results.iter().map(|p| p.len()).min().unwrap();
        results = new_results.into_iter().filter(|p| p.len() == min_len).collect();

        pos = end;
    }
    results
}

fn get_path(start: (i8, i8), end: (i8, i8), lava: (i8, i8), keys: &HashMap<(i8, i8), char>) -> Vec<String> {
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
