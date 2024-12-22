use std::{collections::HashMap, env::args};

use helpers::read_input_file;

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day22", &input_type).unwrap();

    let initial_secret_numbers = contents.lines().map(|line| line.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    const ITERATIONS: usize = 2000;
    let all_secret_numbers = initial_secret_numbers
        .iter()
        .map(|&number| get_secret_numbers::<ITERATIONS>(number))
        .collect::<Vec<Vec<u64>>>();
    println!("Part 1: {}", all_secret_numbers.iter().map(|numbers| numbers.last().unwrap()).sum::<u64>());

    let all_ones = all_secret_numbers
        .iter()
        .map(|numbers| numbers.iter().map(|&number| number as i32 % 10).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let changes = all_ones
        .iter()
        .map(|ones| ones.iter().skip(1).enumerate().map(|(index, one)| one - ones[index]).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let all_ones = all_ones.iter().map(|numbers| numbers.iter().cloned().skip(1).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let sequences = all_ones
        .iter()
        .enumerate()
        .map(|(ones_ix, ones)| {
            let mut sequences: HashMap<[i32; 4], i32> = HashMap::new();
            ones.iter()
                .enumerate()
                .for_each(|(ix, &value)| {
                    if ix >= 3 {
                        let sequence = (changes[ones_ix][ix - 3..=ix].try_into().unwrap(), value);
                        sequences.entry(sequence.0).or_insert(sequence.1);
                    }
                });
            sequences
        })
        .collect::<Vec<_>>();
    
    let mut max = 0;
    let mut best_sequence: [i32; 4] = [0; 4];
    for l1 in -9..=9 {
        for l2 in -9..=9 {
            for l3 in -9..=9 {
                for l4 in -9..=9 {
                    let sequence = [l1, l2, l3, l4];
                    let mut local_max = 0;
                    for s in &sequences {
                        if s.contains_key(&sequence) {
                            local_max += s[&sequence];
                        }
                    }
                    if local_max > max {
                        max = local_max;
                        best_sequence = sequence;
                    }
                }
            }
        }
    }

    println!("Part 2: {} {:?}", max, best_sequence);
}

fn get_secret_numbers<const N: usize>(starting_number: u64) -> Vec<u64> {
    let mut secret_numbers = vec![starting_number];
    let mut next_secret_number = starting_number;
    for _ in 0..N {
        next_secret_number ^= next_secret_number * 64;
        next_secret_number %= 16777216;
        next_secret_number ^= next_secret_number / 32;
        next_secret_number %= 16777216;
        next_secret_number ^= next_secret_number * 2048;
        next_secret_number %= 16777216;

        secret_numbers.push(next_secret_number);
    }

    secret_numbers
}
