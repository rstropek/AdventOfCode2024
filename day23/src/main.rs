use std::{collections::HashMap, collections::HashSet, env::args};

use helpers::read_input_file;

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day23", &input_type).unwrap();

    let graph = parse(contents);
    let triangles = find_triangles(&graph);
    let triangles_m = triangles.iter().filter(|&triangle| triangle.iter().any(|node| node.starts_with("t"))).collect::<Vec<_>>();
    println!("Part 1: {}", triangles_m.len());

    let mut max_set = find_max_set(&graph);
    max_set.sort();
    println!("Part 2: {:?}", max_set.join(","));
}

fn parse(contents: String) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines() {
        let (from, to) = line.split_once('-').unwrap();

        graph.entry(from.to_string()).or_default().push(to.to_string());
        graph.entry(to.to_string()).or_default().push(from.to_string());
    }
    graph
}

fn find_triangles(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut triangles = Vec::new();

    let nodes: Vec<&String> = graph.keys().collect();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            for k in (j + 1)..nodes.len() {
                let node1 = nodes[i];
                let node2 = nodes[j];
                let node3 = nodes[k];

                // Check if all three nodes are connected
                if graph[node1].contains(node2) && graph[node1].contains(node3) && graph[node2].contains(node3) {
                    triangles.push(vec![node1.clone(), node2.clone(), node3.clone()]);
                }
            }
        }
    }

    triangles
}

fn find_max_set(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let nodes: Vec<&String> = graph.keys().collect();
    let mut max_set = Vec::new();

    for &start_node in &nodes {
        let mut current_set = vec![start_node.clone()];
        let mut candidates: HashSet<&String> = graph[start_node].iter().collect();

        extend_set(graph, &mut current_set, &mut candidates);

        if current_set.len() > max_set.len() {
            max_set = current_set;
        }
    }

    max_set
}

fn extend_set(graph: &HashMap<String, Vec<String>>, current_set: &mut Vec<String>, candidates: &mut HashSet<&String>) {
    while !candidates.is_empty() {
        let &next = candidates.iter().next().unwrap();
        candidates.remove(next);

        if current_set.iter().all(|node| graph[next].contains(node)) {
            current_set.push(next.clone());

            let mut new_candidates = candidates.clone();
            new_candidates.retain(|&node| current_set.iter().all(|set_node| graph[node].contains(set_node)));

            if !new_candidates.is_empty() {
                extend_set(graph, current_set, &mut new_candidates);
            }
        }
    }
}
