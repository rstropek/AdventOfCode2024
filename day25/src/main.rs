use std::{collections::HashSet, env::args};
use helpers::read_input_file;

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day25", &input_type).unwrap();

    let locks_keys = contents.split("\n\n").map(|s| s.lines().map(|l| l.as_bytes()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut locks: HashSet<[u8; 5]> = HashSet::new();
    let mut keys: HashSet<[u8; 5]> = HashSet::new();
    
    for lk in locks_keys {
        if lk[0].iter().all(|c| *c == b'#') {
            // It is a lock
            let mut heights = [0; 5];
            for x in 0..5 {
                for y in 1..=6 {
                    if lk[y][x] == b'.' {
                        break;
                    }

                    heights[x] += 1;
                }
            }

            locks.insert(heights);
        } else {
            // It is a key
            let mut heights = [0; 5];
            for x in 0..5 {
                for y in (0..=5).rev() {
                    if lk[y][x] == b'.' {
                        break;
                    }

                    heights[x] += 1;
                }
            }

            keys.insert(heights);
        }
    }

    let mut counter = 0;
    for lock in &locks {
        for key in &keys {
            let mut fits = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    fits = false;
                    break;
                }
            }

            if fits {
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}

