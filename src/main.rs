mod cleaning;
mod filtering;
mod selecting;
mod graph;
mod output;
mod play;

use cleaning::clean_csv;
use filtering::filter_tom_brady;
use selecting::select_columns;
use graph::{build_graph_from_csv, analyze_graph,};
use std::fs;

fn validate_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if fs::metadata(file_path)?.len() == 0 {
        return Err(format!("File {} is empty", file_path).into());
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cleaned_file = "filtered_nfl.csv";
    let brady_file = "tom_brady_pass_attempts.csv";
    let final_file = "brady_filtered_columns.csv";

    let cluster_output_path = "output/clusters";
    let graph_output_path = "output/graphs";

    // Create output directories
    fs::create_dir_all(cluster_output_path).expect("Failed to create clusters directory");
    fs::create_dir_all(graph_output_path).expect("Failed to create graphs directory");

    // Step 1: Clean and process the original dataset
    clean_csv("nfl_pbp_2020_to_2022.csv", cleaned_file)
        .map_err(|e| format!("Error cleaning CSV: {}", e))?;
    validate_file(cleaned_file)?;

    // Step 2: Filter for Tom Brady's passing plays
    filter_tom_brady(cleaned_file, brady_file)
        .map_err(|e| format!("Error filtering for Tom Brady's plays: {}", e))?;
    validate_file(brady_file)?;

    // Step 3: Select only desired columns
    select_columns(brady_file, final_file)
        .map_err(|e| format!("Error selecting desired columns: {}", e))?;
    validate_file(final_file)?;

    println!("Filtered CSV with desired columns created: {}", final_file);

    // Step 4: Graphing
    let csv_path = "brady_filtered_columns.csv";

    // Build the graph
    let graph = build_graph_from_csv(csv_path)?;

    // Analyze the graph
    analyze_graph(&graph);

    Ok(())
}
