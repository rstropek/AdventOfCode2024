use std::collections::HashMap;
use std::env::args;

use helpers::read_input_file;
use regex::Regex;

/* 
    THIS CODE IS HORRIBLE, I know 😅.
    However, I had little time around Christmas and so I just wrote enough code to spot
    the switched gates by partly inspecting the intermediate results and the input.
*/

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Gate {
    operation: Operation,
    input1: String,
    input2: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Wire {
    name: String,
    state: Option<bool>,
    gate: Option<Gate>,
}

fn parse_input(contents: &str) -> HashMap<String, Wire> {
    let mut wires = HashMap::new();

    let initial_state_re = Regex::new(r"^([a-z]\d{2}|[a-z]{3}):\s*(\d)$").unwrap();
    let gate_re = Regex::new(r"^([a-z]\d{2}|[a-z]{3})\s+(AND|OR|XOR)\s+([a-z]\d{2}|[a-z]{3})\s+->\s+([a-z]\d{2}|[a-z]{3})$").unwrap();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        if let Some(caps) = initial_state_re.captures(line) {
            let wire_name = caps[1].to_string();
            let state = caps[2].parse::<u8>().unwrap() == 1;

            wires.insert(
                wire_name.clone(),
                Wire {
                    name: wire_name.clone(),
                    state: Some(state),
                    gate: None,
                },
            );
        } else if let Some(caps) = gate_re.captures(line) {
            let input1 = caps[1].to_string();
            let operation = match &caps[2] {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => unreachable!(),
            };
            let input2 = caps[3].to_string();
            let output = caps[4].to_string();

            wires.insert(
                output.clone(),
                Wire {
                    name: output.clone(),
                    state: None,
                    gate: Some(Gate { operation, input1, input2 }),
                },
            );
        }
    }

    wires
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Gate2 {
    input1: String,
    operation: Operation,
    input2: String,
    output: String,
}

impl Gate2 {
    fn new(input1: String, operation: Operation, input2: String, output: String) -> Self {
        Self { input1, operation, input2, output }
    }

    fn as_x_y_input(&self) -> Option<u8> {
        if (self.input1.starts_with("x") && self.input2.starts_with("y")) || (self.input1.starts_with("y") && self.input2.starts_with("x")) {
            let ix1 = self.input1[1..].parse::<u8>();
            let ix2 = self.input2[1..].parse::<u8>();
            if let (Ok(n1), Ok(n2)) = (ix1, ix2) {
                if n1 == n2 {
                    return Some(n1);
                }
            }
        }

        None
    }

    fn as_z_output(&self) -> Option<u8> {
        if self.output.starts_with("z") {
            let ix = self.output[1..].parse::<u8>();
            if ix.is_ok() {
                return Some(ix.unwrap());
            }
        }

        None
    }
}

fn parse_input2(contents: &str) -> Vec<Gate2> {
    let mut gates = Vec::new();

    let gate_re = Regex::new(r"^([a-z]\d{2}|[a-z]{3})\s+(AND|OR|XOR)\s+([a-z]\d{2}|[a-z]{3})\s+->\s+([a-z]\d{2}|[a-z]{3})$").unwrap();

    for line in contents.lines() {
        if line.is_empty() || !line.contains(" -> ") {
            continue;
        }

        let caps = gate_re.captures(line).unwrap();
        let input1 = caps[1].to_string();
        let operation = match &caps[2] {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => unreachable!(),
        };
        let input2 = caps[3].to_string();
        let output = caps[4].to_string();

        gates.push(Gate2 { operation, input1, input2, output });
    }

    gates
}

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day24", &input_type).unwrap();
    let wires = parse_input(&contents);

    {
        let mut wires = wires.clone();
        while wires.iter().any(|(name, wire)| name.starts_with("z") && wire.state.is_none()) {
            let wire_names: Vec<String> = wires.keys().cloned().collect();
            for name in wire_names {
                if let Some(wire) = wires.get(&name) {
                    if wire.state.is_none() {
                        let gate = wire.gate.as_ref().unwrap();
                        let input1 = wires.get(&gate.input1).unwrap().state;
                        let input2 = wires.get(&gate.input2).unwrap().state;

                        if input1.is_none() || input2.is_none() {
                            continue;
                        }

                        let output = match gate.operation {
                            Operation::And => input1.unwrap() && input2.unwrap(),
                            Operation::Or => input1.unwrap() || input2.unwrap(),
                            Operation::Xor => input1.unwrap() ^ input2.unwrap(),
                        };

                        wires.get_mut(&name).unwrap().state = Some(output);
                    }
                }
            }
        }

        let mut z_wires: Vec<_> = wires.iter().filter(|(name, _)| name.starts_with("z")).collect();
        z_wires.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));

        let mut result: u64 = 0;
        for (name, wire) in z_wires {
            let z_index = name[1..].parse::<usize>().unwrap();
            result |= wire.state.unwrap() as u64 * 2_u64.pow(z_index as u32);
        }

        println!(" RESULT {}", result);
    }

    let gates = parse_input2(&contents);

    let mut switched = Vec::new();

    // find all gates that result in z and are not xor
    let z_gates: Vec<_> = gates.iter().filter(|g| g.output.starts_with("z"))
        .filter(|g| g.operation != Operation::Xor && g.as_z_output().unwrap() < 45)
        .collect();

    for z in &z_gates {
        println!("{:?}", z);
    }
    println!();

    let prelim: Vec<_> = gates.iter()
        .enumerate()
        .filter_map(|(ix, g)| if g.operation == Operation::Xor && g.as_x_y_input().is_some() { Some((ix, g.as_x_y_input().unwrap())) } else { None })
        .map(|(ix, n)| (gates[ix].output.clone(), n))
        .collect();

    for gate in z_gates {
        let z_ix = gate.as_z_output().unwrap();
        let prelim_gate = prelim.iter()
            .filter(|(_, n)| *n == z_ix)
            .map(|(g, _)| g)
            .next()
            .unwrap();

        let correct_gate = gates.iter()
            .filter(|g| g.operation == Operation::Xor && (g.input1 == *prelim_gate || g.input2 == *prelim_gate))
            .next()
            .unwrap();

        switched.push(gate.output.clone());
        switched.push(correct_gate.output.clone());

        //println!("{:?} -> {:?}", gate.output, correct_gate);
    }

    let carry: Vec<_> = gates.iter()
        .enumerate()
        .filter_map(|(ix, g)| if g.operation == Operation::And && g.as_x_y_input().is_some() { Some((ix, g.as_x_y_input().unwrap())) } else { None })
        .map(|(ix, n)| (gates[ix].output.clone(), n + 1))
        .collect();

    for c in &carry {
        let carry_or = gates.iter()
            .filter(|g| g.operation == Operation::Or && (g.input1 == c.0 || g.input2 == c.0))
            .next();

        println!("{:?} -> {:?}", c.0, carry_or);
    }


    for c in &carry {
        println!("{:?}", c);
    }

    switched.push("fgt".to_string());
    switched.push("pcp".to_string());

    switched.sort();
    println!("{:?}", switched.join(","));
}

fn get_preliminary_sum_gate(wires: &HashMap<String, Wire>, z_ix: u8) -> &Wire {
    wires
        .values()
        .filter(|v| v.gate.is_some())
        .filter(|v| {
            ((v.gate.as_ref().unwrap().input1.starts_with("x") && v.gate.as_ref().unwrap().input2.starts_with("y"))
                || (v.gate.as_ref().unwrap().input1.starts_with("y") && v.gate.as_ref().unwrap().input2.starts_with("x")))
                && v.gate.as_ref().unwrap().operation == Operation::Xor
        })
        .filter(|v| split_gate_name(&v.gate.as_ref().unwrap().input1).unwrap().1 == z_ix && split_gate_name(&v.gate.as_ref().unwrap().input2).unwrap().1 == z_ix)
        .next()
        .unwrap()
}

fn get_carry_from_prev_gate(wires: &HashMap<String, Wire>, z_ix: u8) -> &Wire {
    wires
        .values()
        .filter(|v| v.gate.is_some())
        .filter(|v| {
            ((v.gate.as_ref().unwrap().input1.starts_with("x") && v.gate.as_ref().unwrap().input2.starts_with("y"))
                || (v.gate.as_ref().unwrap().input1.starts_with("y") && v.gate.as_ref().unwrap().input2.starts_with("x")))
                && v.gate.as_ref().unwrap().operation == Operation::And
        })
        .filter(|v| split_gate_name(&v.gate.as_ref().unwrap().input1).unwrap().1 == z_ix - 1 && split_gate_name(&v.gate.as_ref().unwrap().input2).unwrap().1 == z_ix - 1)
        .next()
        .unwrap()
}

fn split_gate_name(name: &str) -> Option<(char, u8)> {
    let ix = name[1..].parse::<u8>();
    if ix.is_err() {
        return None;
    }

    let c = name.as_bytes()[0];
    Some((c as char, ix.unwrap()))
}
