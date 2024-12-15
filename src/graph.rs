use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use petgraph::algo::connected_components;
use std::collections::HashMap;
use crate::play::Play;

pub fn build_graphs_by_down_and_distance(csv_path: &str) -> HashMap<(u8, String), Graph<String, u32, Undirected>> {
    let mut graphs: HashMap<(u8, String), Graph<String, u32, Undirected>> = HashMap::new();
    let mut rdr = csv::Reader::from_path(csv_path).expect("Failed to read CSV");

    for (line_number, result) in rdr.deserialize::<Play>().enumerate() {
        match result {
            Ok(mut record) => {
                if record.play_id.is_none() {
                    eprintln!("Skipping row {}: Missing or invalid play_id", line_number + 1);
                    continue; // Skip the row
                }
    
                record.categorize_distance(); // Populate distance_category
    
                if let (Some(down), Some(distance_category), Some(receiver)) =
                    (record.down, record.distance_category.clone(), Some(record.receiver.clone()))
                {
                    let key = (down, distance_category);
    
                    let graph = graphs.entry(key).or_insert_with(|| Graph::<String, u32, Undirected>::new_undirected());
    
                    let passer_index = graph.add_node("T.Brady".to_string());
                    let receiver_index = graph.add_node(receiver);
    
                    if let Some(edge) = graph.find_edge(passer_index, receiver_index) {
                        if let Some(weight) = graph.edge_weight(edge) {
                            graph.update_edge(passer_index, receiver_index, weight + 1);
                        }
                    } else {
                        graph.update_edge(passer_index, receiver_index, 1);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to deserialize row {}: {}", line_number + 1, err);
            }
        }
    }    
    graphs
}
pub fn cluster_graph(graph: &Graph<String, u32, Undirected>) -> HashMap<NodeIndex, usize> {
    let mut node_to_cluster = HashMap::new();
    let cluster_count = connected_components(&graph);
    
    for (index, node) in graph.node_indices().enumerate() {
        node_to_cluster.insert(node, index % cluster_count); // Assign nodes to clusters
    }

    node_to_cluster
}

