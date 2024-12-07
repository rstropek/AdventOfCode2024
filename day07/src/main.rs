use anyhow::Result;
use helpers::read_input_file;
use std::env::args;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn solve(intermediate_result: u64, result: u64, operands: &[u64], with_combine: bool) -> bool {
    if operands.is_empty() {
        return intermediate_result == result;
    }

    if operands.len() == 1 && (result == intermediate_result * operands[0] 
        || result == intermediate_result / operands[0] 
        || (with_combine && result == combine_op(intermediate_result, operands[0]))) {
        return true;
    }

    solve(intermediate_result * operands[0], result, &operands[1..], with_combine)
        || solve(intermediate_result + operands[0], result, &operands[1..], with_combine)
        || (with_combine && solve(combine_op(intermediate_result, operands[0]), result, &operands[1..], with_combine))
}

fn part(equations: &[Equation], with_combine: bool) {
    let mut sum = 0;
    for equation in equations {
        if solve(equation.operands[0] + equation.operands[1], equation.result, &equation.operands[2..], with_combine)
            || solve(equation.operands[0] * equation.operands[1], equation.result, &equation.operands[2..], with_combine)
            || (with_combine && solve(combine_op(equation.operands[0], equation.operands[1]), equation.result, &equation.operands[2..], with_combine))
        {
            sum += equation.result;
        }
    }

    println!("Sum: {}", sum);
}

fn combine_op(v1: u64, v2: u64) -> u64{
    v1 * 10u64.pow((v2 as f64).log10().floor() as u32 + 1) + v2
}

fn main() -> Result<()> {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day07", &input_type)?;

    let equations = contents
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            let result = parts[0].parse::<u64>().unwrap();
            let operands = parts[1].split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
            Equation { result, operands }
        })
        .collect::<Vec<_>>();


    part(&equations, false);
    part(&equations, true);
    Ok(())
}
