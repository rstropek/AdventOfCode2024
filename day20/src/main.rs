use helpers::{read_input_file, SquareText, DIRECTIONS_USIZE};
use std::{
    collections::{HashMap, HashSet},
    env::args,
};

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day20", &input_type).unwrap();

    let mut contents: Vec<Vec<u8>> = contents.lines().map(|line| line.as_bytes().to_vec()).collect();
    let start = contents.find_byte(b'S');
    let end = contents.find_byte(b'E');
    contents[start.1][start.0] = b'.';
    contents[end.1][end.0] = b'.';

    let track = find_track(&contents, start, end);

    const CHEAT_DURATION_1: usize = 2;
    let shortcuts = find_all_shortcuts::<CHEAT_DURATION_1>(&contents, &track);
    let cheats = get_cheats(&contents, &track, &shortcuts);
    let count = cheats.iter().filter(|cheat| cheat.saving >= 100).count();
    println!("{} cheats save at least 100", count);

    const CHEAT_DURATION_2: usize = 20;
    let shortcuts = find_all_shortcuts::<CHEAT_DURATION_2>(&contents, &track);
    let cheats = get_cheats(&contents, &track, &shortcuts);
    let count = cheats.iter().filter(|cheat| cheat.saving >= 100).count();
    println!("{} cheats save at least 100", count);
}

fn get_cheats(contents: &[Vec<u8>], track: &HashMap<(usize, usize), usize>, all_cheats: &HashMap<Cheat, usize>) -> HashSet<CheatWithSavings> {
    let mut cheats = HashSet::new();
    for (&start, &start_duration) in track {
        for y in 0..contents.get_height() {
            for x in 0..contents.get_width() {
                if let Some(end_duration) = all_cheats.get(&Cheat { start, end: (x, y) }) {
                    let manhattan_dist = start.0.abs_diff(x) + start.1.abs_diff(y);
                    if end_duration + manhattan_dist < start_duration {
                        cheats.insert(CheatWithSavings {
                            start,
                            end: (x, y),
                            saving: start_duration - end_duration - manhattan_dist,
                        });
                    }
                }
            }
        }
    }
    cheats
}

fn find_track(contents: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut current = start;
    let mut previous = None;
    let mut track = vec![];

    while current != end {
        track.push(current);

        let mut next = current;
        for direction in DIRECTIONS_USIZE {
            let (x, y) = (current.0.wrapping_add(direction.0), current.1.wrapping_add(direction.1));
            if contents[y][x] == b'.' && (previous.is_none() || previous.unwrap() != (x, y)) {
                next = (x, y);
            }
        }

        previous = Some(current);
        current = next;
    }

    track.push(current);
    track.iter().enumerate().map(|(i, &pos)| (pos, track.len() - i - 1)).collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CheatWithSavings {
    start: (usize, usize),
    end: (usize, usize),
    saving: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cheat {
    start: (usize, usize),
    end: (usize, usize),
}

fn find_all_shortcuts<const N: usize>(contents: &[Vec<u8>], track: &HashMap<(usize, usize), usize>) -> HashMap<Cheat, usize> {
    let mut cheats = HashMap::new();

    for &pos in track.keys() {
        for direction in DIRECTIONS_USIZE {
            let (x, y) = (pos.0.wrapping_add(direction.0), pos.1.wrapping_add(direction.1));
            if contents[y][x] == b'#' {
                for y2 in 0..contents.get_height() {
                    for x2 in 0..contents.get_width() {
                        let manhattan_dist = pos.0.abs_diff(x2) + pos.1.abs_diff(y2);
                        if manhattan_dist <= N && contents[y2][x2] == b'.' {
                            let cheat = Cheat {
                                start: (pos.0, pos.1),
                                end: (x2, y2),
                            };
                            let new_value = *track.get(&(x2, y2)).unwrap();
                            cheats.entry(cheat).and_modify(|e: &mut usize| *e = (*e).min(new_value)).or_insert(new_value);
                        }
                    }
                }
            }
        }
    }

    cheats
}
