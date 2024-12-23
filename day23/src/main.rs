use std::{collections::HashMap, env::args, collections::HashSet};

use helpers::read_input_file;

fn main() {
    let input_type = args().nth(1).unwrap_or("test".to_string());
    let contents = read_input_file("day23", &input_type).unwrap();

    let graph = parse(contents);
    
    println!("Part 1: {}", find_triangles(&graph).len());

    let mut max_clique = find_max_clique(&graph);

    max_clique.sort();

    println!("Maximum clique: {:?}", max_clique.join(","));

}

fn parse(contents: String) -> HashMap<String, Vec<String>> {
    // Create adjacency list representation using HashMap
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    // Parse each line into connections
    for line in contents.lines() {
        let (from, to) = line.split_once('-').unwrap();
    
        // Add bidirectional connections
        graph.entry(from.to_string())
            .or_default()
            .push(to.to_string());
        graph.entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }
    graph
}

fn find_triangles(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut triangles = Vec::new();
    
    // Convert node names to a sorted vector for consistent iteration
    let nodes: Vec<&String> = graph.keys().collect();
    
    // Check each possible combination of three nodes
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            for k in (j + 1)..nodes.len() {
                let node1 = nodes[i];
                let node2 = nodes[j];
                let node3 = nodes[k];
                
                // Check if all three nodes are connected to each other
                if graph[node1].contains(node2) && 
                   graph[node1].contains(node3) && 
                   graph[node2].contains(node3) {
                    triangles.push(vec![
                        node1.clone(),
                        node2.clone(),
                        node3.clone(),
                    ]);
                }
            }
        }
    }
    
    triangles
}

fn find_max_clique(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let nodes: Vec<&String> = graph.keys().collect();
    let mut max_clique = Vec::new();
    
    // Try each node as a starting point
    for &start_node in &nodes {
        let mut current_clique = vec![start_node.clone()];
        let mut candidates: HashSet<&String> = graph[start_node].iter().collect();
        
        extend_clique(graph, &mut current_clique, &mut candidates);
        
        if current_clique.len() > max_clique.len() {
            max_clique = current_clique;
        }
    }
    
    max_clique
}

fn extend_clique(
    graph: &HashMap<String, Vec<String>>,
    current_clique: &mut Vec<String>,
    candidates: &mut HashSet<&String>
) {
    while !candidates.is_empty() {
        let &next = candidates.iter().next().unwrap();
        candidates.remove(next);
        
        // Check if this node connects to all nodes in current clique
        if current_clique.iter().all(|node| graph[next].contains(node)) {
            current_clique.push(next.clone());
            
            // Update candidates to only include nodes connected to everything
            let mut new_candidates = candidates.clone();
            new_candidates.retain(|&node| 
                current_clique.iter().all(|clique_node| graph[node].contains(clique_node))
            );
            
            if !new_candidates.is_empty() {
                extend_clique(graph, current_clique, &mut new_candidates);
            }
        }
    }
}
