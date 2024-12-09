use std::{collections::HashSet, env::args};

use anyhow::Result;
use helpers::read_input_file;

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day09", &input_type)?;
    let content_bytes = contents.as_bytes();

    part1(content_bytes);
    part2(content_bytes);
    Ok(())
}

fn part1(content_bytes: &[u8]) {
    let mut front_ix = 0;
    let mut back_ix = content_bytes.len() - 1;
    let mut checksum = 0u64;
    let mut block_ix = 0u64;
    let mut front_file_id = 0u64;
    let mut back_file_id = content_bytes.len() as u64 / 2u64;
    let mut back_added = 0;
    let mut back_length: u8 = content_bytes[content_bytes.len() - 1] - b'0';
    loop {
        if front_file_id >= back_file_id {
            break;
        }

        let front_length: u8 = content_bytes[front_ix] - b'0';

        for _ in 0..front_length {
            checksum += block_ix * front_file_id;
            block_ix += 1;
        }

        front_file_id += 1;

        let mut empty = content_bytes[front_ix + 1] - b'0';
        while empty > 0 {
            if back_length - back_added > 0 {
                checksum += block_ix * back_file_id;
                block_ix += 1;
                back_added += 1;
                empty -= 1;
            } else {
                back_file_id -= 1;
                back_ix -= 2;
                back_length = content_bytes[back_ix] - b'0';
                back_added = 0;
            }
        }

        front_ix += 2;
    }

    for _ in 0..back_length - back_added {
        checksum += block_ix * back_file_id;
        block_ix += 1;
    }

    println!("\n\n{}", checksum);
}

fn part2(content_bytes: &[u8]) {
    let mut moved = HashSet::new();

    let mut front_ix = 0;
    let mut checksum = 0u64;
    let mut block_ix = 0u64;
    let mut front_file_id = 0u64;
    while front_ix < content_bytes.len() {
        if !moved.contains(&front_file_id) {
            let front_length: u8 = content_bytes[front_ix] - b'0';

            for _ in 0..front_length {
                checksum += block_ix * front_file_id;
                block_ix += 1;
            }
        } else {
            let empty = content_bytes[front_ix] - b'0';
            handle_empty(content_bytes, empty, front_ix, &mut moved, &mut checksum, &mut block_ix);
        }

        front_file_id += 1;

        if front_ix + 1 >= content_bytes.len() {
            break;
        }

        let empty = content_bytes[front_ix + 1] - b'0';
        handle_empty(content_bytes, empty, front_ix, &mut moved, &mut checksum, &mut block_ix);

        front_ix += 2;
    }

    println!("\n\n{}", checksum);
}

fn handle_empty(content_bytes: &[u8], mut empty: u8, front_ix: usize, moved: &mut HashSet<u64>, checksum: &mut u64, block_ix: &mut u64) {
    let mut back_ix = content_bytes.len() - 1;
    let mut back_file_id = content_bytes.len() as u64 / 2u64;
    let mut back_length = content_bytes[back_ix] - b'0';
    while empty > 0 && back_ix > front_ix {
        if !moved.contains(&back_file_id) && empty >= back_length {
            for _ in 0..back_length {
                *checksum += *block_ix * back_file_id;
                *block_ix += 1;
            }

            empty -= back_length;
            moved.insert(back_file_id);
        } else {
            back_file_id -= 1;
            back_ix -= 2;
            back_length = content_bytes[back_ix] - b'0';
        }
    }

    for _ in 0..empty {
        *block_ix += 1;
    }
}
