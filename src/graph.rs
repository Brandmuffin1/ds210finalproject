use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::collections::HashMap;

pub type ReceiverGraph = Graph<String, (), Directed>;

// Build a graph where each pass attempt is a vertex and connects to a receiver node
pub fn build_graph_from_csv(
    csv_path: &str,
) -> Result<ReceiverGraph, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;
    let mut graph = ReceiverGraph::new();
    let mut receiver_nodes: HashMap<String, NodeIndex> = HashMap::new();
    for (line_number, result) in rdr.records().enumerate() {
        let record = result?;
        let receiver = record.get(6).unwrap_or("").to_string(); // Column 6 is "receiver"
        if receiver.is_empty() {
            eprintln!("Skipping row {}: Missing `receiver` field", line_number + 1);
            continue;
        }
        
        // Add or retrieve the node for the receiver
        let receiver_index = *receiver_nodes.entry(receiver.clone()).or_insert_with(|| graph.add_node(receiver.clone()));
        
        // Add a vertex for the individual pass attempt and connect it to the receiver
        let pass_attempt_vertex = format!("Pass {}", line_number + 1); // Unique identifier for the pass
        let pass_attempt_index = graph.add_node(pass_attempt_vertex);
        graph.add_edge(pass_attempt_index, receiver_index, ());
    }
    Ok(graph)
}

// Analyze the graph with detailed information about nodes and edges
pub fn analyze_graph(graph: &ReceiverGraph) {
    println!("Graph Info:");
    println!("Nodes: {}", graph.node_count());
    println!("Edges: {}", graph.edge_count());
    
    // Track incompletions and penalties
    println!("\nPass -> Receiver:");
    let mut incompletions = 0;

    // List of every pass to its respective receiver
    println!("\nPass -> Receiver:");
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let source_label = graph[source].clone();
        let target_label = graph[target].clone();

        // Ensure correct direction (Pass -> Receiver)
        if source_label.starts_with("Pass") {
            if target_label == "0" {
                println!("{} -> Incomplete/Penalty", source_label);
                incompletions += 1;
            } else {
                println!("{} -> {}", source_label, target_label);
            }
        } else if target_label.starts_with("Pass") {
            if source_label == "0" {
                println!("{} -> Incomplete/Penalty", target_label);
                incompletions += 1;
            } else {
                println!("{} -> {}", target_label, source_label);
            }
        }
    }
    
    // Count unique receivers
    let unique_receivers: Vec<String> = graph
        .node_indices()
        .filter_map(|node| {
            let node_label = graph[node].clone();
            if !node_label.starts_with("Pass") {
                Some(node_label)
            } else {
                None
            }
        })
        .collect();

    println!("\nNumber of Unique Receivers Targeted: {}", unique_receivers.len());

    // Print total incompletions
    println!("\nTotal Incompletions: {}", incompletions);

    // Count the number of edges (targets) for each receiver
    let mut receiver_counts: HashMap<String, usize> = HashMap::new();
    for node in graph.node_indices() {
        if let Some(receiver_name) = graph.node_weight(node) {
            if !receiver_name.starts_with("Pass") {
                let count = graph.edges_directed(node, petgraph::Incoming).count();
                receiver_counts.insert(receiver_name.clone(), count);
            }
        }
    }

    // Sort receiver counts in descending order
    let mut sorted_counts: Vec<(String, usize)> = receiver_counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count in descending order

    // Print receiver target counts
    println!("\nReceiver Target Counts (Greatest to Least):");
    for (receiver, count) in sorted_counts {
        println!("Receiver: {}, Targets: {}", receiver, count);
    }
}