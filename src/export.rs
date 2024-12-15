use petgraph::dot::{Dot, Config};
use csv::Writer;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

pub fn export_clusters_to_csv(
    clusters: HashMap<NodeIndex, usize>, 
    graph: &Graph<String, u32, Undirected>, 
    down: u8, 
    distance_category: &str, 
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path(output_path)?;

    // Write the CSV header
    wtr.write_record(&["Down", "Distance", "Cluster", "Player"])?;

    for (node, cluster_id) in clusters {
        let player = graph[node].clone(); // Get player name from node index
        wtr.write_record(&[down.to_string(), distance_category.to_string(), cluster_id.to_string(), player])?;
    }

    wtr.flush()?;
    Ok(())
}
pub fn export_graph_to_dot(
    graph: &Graph<String, u32, Undirected>, 
    down: u8, 
    distance_category: &str, 
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let file_name = format!("{}/graph_down_{}_distance_{}.dot", output_path, down, distance_category);
    std::fs::write(file_name, format!("{}", dot))?;
    Ok(())
}
