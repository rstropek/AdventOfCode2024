use helpers::{read_input_file, SquareText, DIRECTIONS_USIZE};
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

    const CHEAT_DURATION_1: usize = 2;
    let track = find_track(&contents, start, end);
    let mut all_cheats = HashSet::new();
    for (p, _) in &track {
        let cheats = find_shortcuts::<CHEAT_DURATION_1>(&contents, &track, *p);
        all_cheats.extend(cheats);
    }

    //let count = all_cheats.iter().filter(|cheat| cheat.saving >= 100).count();
    //println!("{} cheats save at least 100", count);

    const CHEAT_DURATION_2: usize = 20;
    let all_cheats = find_all_cheats::<CHEAT_DURATION_2>(&contents, &track);

    let interesting_cheat = all_cheats.get(&Cheat2 { start: (1, 4), end: (3, 7)});
    println!("cheat {:?}", interesting_cheat);
    let interesting_start = track.get(&(1, 3)).unwrap();
    println!("start {:?}", interesting_start);
    
    let mut cheats = HashSet::new();
    for (&start, &start_duration) in &track {
        for direction in DIRECTIONS_USIZE {
            let (x, y) = (start.0.wrapping_add(direction.0), start.1.wrapping_add(direction.1));
            if contents[y][x] == b'#' && !cheats.iter().any(|c: &Cheat| c.start == (x, y)) {
                for cheat in all_cheats.keys().filter(|cheat| cheat.start == (x, y)) {
                    let end_duration = all_cheats[cheat];
                    let manhattan_dist = x.abs_diff(cheat.end.0) + y.abs_diff(cheat.end.1) + 1;
                    if end_duration + manhattan_dist < start_duration {
                        cheats.insert(Cheat {
                            start: (x, y),
                            end: cheat.end,
                            saving: start_duration - end_duration - manhattan_dist,
                        });
                    }
                }
            }
        }
    }

    for cheat in &cheats {
        if cheat.saving == 76 {
            println!("76: Cheat from {:?} to {:?} saves {}", cheat.start, cheat.end, cheat.saving);
        }
    }

    let mut cheats_by_saving = HashMap::new();
    for cheat in cheats {
        *cheats_by_saving.entry(cheat.saving).or_insert(0) += 1;
    }

    let mut savings: Vec<_> = cheats_by_saving.iter().collect();
    savings.sort_by_key(|(&saving, _)| saving);
    for (&saving, count) in savings {
        println!("{1} cheats save {0}", saving, count);
    }

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
struct Cheat {
    start: (usize, usize),
    end: (usize, usize),
    saving: usize,
}

struct CheatStep {
    start: (usize, usize),
    current: (usize, usize),
    duration: usize,
}

fn find_shortcuts<const N: usize>(contents: &[Vec<u8>], track: &HashMap<(usize, usize), usize>, start: (usize, usize)) -> HashSet<Cheat> {
    let mut q = VecDeque::new();
    for direction in DIRECTIONS_USIZE {
        let (x, y) = (start.0.wrapping_add(direction.0), start.1.wrapping_add(direction.1));
        if contents[y][x] == b'#' {
            q.push_back(CheatStep {
                start: (x, y),
                current: (x, y),
                duration: 1,
            });
        }
    }

    let mut cheats = HashSet::new();
    while let Some(cheat_step) = q.pop_front() {
        if cheat_step.duration == N {
            if let Some(&end_duration) = track.get(&cheat_step.current) {
                let start_duration = *track.get(&start).unwrap();
                if end_duration + cheat_step.duration < start_duration {
                    cheats.insert(Cheat {
                        start: cheat_step.start,
                        end: cheat_step.current,
                        saving: start_duration - end_duration - cheat_step.duration,
                    });
                }
            }
        } else {
            for direction in DIRECTIONS_USIZE {
                let (x, y) = (cheat_step.current.0.wrapping_add(direction.0), cheat_step.current.1.wrapping_add(direction.1));
                if x < contents.get_width() && y < contents.get_height() {
                    q.push_back(CheatStep {
                        start: cheat_step.start,
                        current: (x, y),
                        duration: cheat_step.duration + 1,
                    });
                }
            }
        }
    }

    cheats
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cheat2 {
    start: (usize, usize),
    end: (usize, usize),
}

fn find_all_cheats<const N: usize>(contents: &[Vec<u8>], track: &HashMap<(usize, usize), usize>) -> HashMap<Cheat2, usize> {
    let mut cheats = HashMap::new();
    for y in 0..contents.get_height() {
        for x in 0..contents.get_width() {
            if contents[y][x] == b'#' {
                for y2 in 0..contents.get_height() {
                    for x2 in 0..contents.get_width() {
                        if x == 10 && y == 7 && x2 == 11 && y2 == 7 {
                            println!("{} {}", x2, y2);
                        }

                        let manhattan_dist = x.abs_diff(x2) + y.abs_diff(y2);
                        if manhattan_dist <= N - 1 && contents[y2][x2] == b'.' {
                            let cheat = Cheat2 {
                                start: (x, y),
                                end: (x2, y2),
                            };
                            let new_value = *track.get(&(x2, y2)).unwrap();
                            cheats.entry(cheat)
                                .and_modify(|e: &mut usize| *e = (*e).min(new_value))
                                .or_insert(new_value);
                        }
                    }
                }
            }
        }
    }

    cheats
}
