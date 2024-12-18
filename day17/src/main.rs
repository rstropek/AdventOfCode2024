use anyhow::Result;
use helpers::read_input_file;
use std::env::args;

type Register = u64;

#[derive(Debug, Clone)]
struct Computer {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    ip: usize,
}

impl Computer {
    fn combo(&self, program: &[u64]) -> Register {
        let operand = program[self.ip + 1];
        match operand {
            0..=3 => operand as Register,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand: {}", operand),
        }
    }
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day17", &input_type)?;

    let (computer, program) = parse_program(&contents);

    let output = run(computer.clone(), &program);
    let output_string = output
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Part 1: {}", output_string);

    let mut starts = vec![0];
    let mut len = 0;
    while len < program.len() - 1 {
        let mut new_starts = Vec::new();
        for start in starts {
            for a in start..start + 8 {
                let mut computer = computer.clone();
                computer.register_a = a;
                let output = run(computer, &program);
                if output
                    == program
                        .clone()
                        .into_iter()
                        .rev()
                        .take(len + 1)
                        .rev()
                        .collect::<Vec<u64>>()
                {
                    new_starts.push(8 * start + 8 * (a - start));
                }
            }
        }
        starts = new_starts;
        len += 1;
    }

    let mut min = u64::MAX;
    for start in starts {
        for a in start - 8..start + 16 {
            let computer = Computer {
                register_a: a,
                register_b: 0,
                register_c: 0,
                ip: 0,
            };
            if run2(computer, &program) && a < min {
                min = a;
            }
        }
    }

    println!("Part 2: {}", min);


    Ok(())
}

fn run(mut computer: Computer, program: &[u64]) -> Vec<Register> {
    let mut output = Vec::new();
    while computer.ip < program.len() {
        let opcode_output = execute(program, &mut computer);
        if let Some(opcode_output) = opcode_output {
            output.push(opcode_output);
        }
    }

    output
}

fn run2(mut computer: Computer, program: &[u64]) -> bool {
    let mut index = 0;
    while computer.ip < program.len() && index < program.len() {
        let opcode_output = execute(program, &mut computer);
        if let Some(opcode_output) = opcode_output {
            if program[index] != opcode_output {
                return false;
            }
            index += 1;
        }
    }

    index == program.len()
}

fn execute(program: &[u64], computer: &mut Computer) -> Option<Register> {
    let instruction = program[computer.ip];

    let mut output = None;
    let mut jump = false;
    match instruction {
        0 /* adv */ => computer.register_a /= 2u64.pow(computer.combo(program) as u32),
        1 /* bxl */ => computer.register_b ^= program[computer.ip + 1],
        2 /* bst */ => computer.register_b = computer.combo(program) % 8,
        3 /* jnz */ => if computer.register_a != 0 {
            computer.ip = computer.combo(program) as usize;
            jump = true;
        },
        4 /* bxc */ => computer.register_b ^= computer.register_c,
        5 /* out */ => output = Some(computer.combo(program) % 8),
        6 /* bdv */ => computer.register_b /= 2u64.pow(computer.combo(program) as u32),
        7 /* cdv */ => computer.register_c = computer.register_a / 2u64.pow(computer.combo(program) as u32),
        _ => panic!("Invalid instruction: {}", instruction),
    }

    if !jump {
        computer.ip += 2;
    }

    output
}

fn parse_program(contents: &str) -> (Computer, Vec<u64>) {
    // Parse register values
    let registers: Vec<Register> = contents
        .lines()
        .take(3)
        .map(|line| line.split(": ").nth(1).unwrap().parse().unwrap())
        .collect();

    // Parse program
    let program: Vec<u64> = contents
        .lines()
        .last()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();

    (
        Computer {
            register_a: registers[0],
            register_b: registers[1],
            register_c: registers[2],
            ip: 0,
        },
        program,
    )
}
