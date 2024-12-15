use serde::Deserialize;
use petgraph::algo::connected_components;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use petgraph::Undirected;

#[derive(Debug, Deserialize)]
pub struct Play {
    pub game_id: String,               // Game ID
    pub play_id: Option<f64>,          // Play ID as an optional field
    #[serde(deserialize_with = "float_to_u8")]
    pub down: u8,                      // Down as integer
    #[serde(deserialize_with = "float_to_u8")]
    pub ydstogo: u8,                   // Yards to go
    pub play_type: String,             // Play type
    pub passer: String,                // Passer
    pub receiver: String,              // Receiver
    pub yards_gained: f32,             // Yards gained
    #[serde(skip_deserializing)]       // Skip deserializing this calculated field
    pub distance_category: Option<String>,
}

impl Play {
    fn cluster_graph(graph: &Graph<String, u32, Undirected>) -> HashMap<NodeIndex, usize> {
        let mut node_to_cluster = HashMap::new();
        let cluster_count = connected_components(&graph);
        
        for (index, node) in graph.node_indices().enumerate() {
            node_to_cluster.insert(node, index % cluster_count); // Assign nodes to clusters
        }
    
        node_to_cluster
    }
    pub fn categorize_distance(&mut self) {
        self.distance_category = match self.yards_gained {
            Some(0.0..=3.0) => Some("Short".to_string()),
            Some(4.0..=7.0) => Some("Medium".to_string()),
            Some(7.0..) => Some("Long".to_string()),
            _ => None,
        };
    }
}
