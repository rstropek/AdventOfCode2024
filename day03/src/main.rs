use anyhow::Result;
use helpers::read_input_file;
use std::env::args;
use regex::Regex;

#[derive(Debug)]	
enum Part {
    One,
    Two,
}

fn solve(contents: &str, part: &Part) -> Result<()> {
    let re;
    match part {
        Part::One => re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?,
        Part::Two => re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?,
    };

    let mut enabled = true;
    let sum: i32 = re.captures_iter(&contents)
        .filter_map(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                None
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
                None
            } else if let Some(n1) = cap.get(1) {
                // This is a mul() pattern
                if enabled {
                    let n1: i32 = n1.as_str().parse().unwrap();
                    let n2: i32 = cap[2].parse().unwrap();
                    Some(n1 * n2)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum();

    println!("Total sum part {part:?}: {sum}");

    Ok(())
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day03", &input_type)?;
    solve(&contents, &Part::One)?;
    solve(&contents, &Part::Two)?;
    Ok(())
}
