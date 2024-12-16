mod cleaning;
mod filtering;
mod selecting;
mod graph;

use cleaning::clean_csv;
use filtering::filter_tom_brady;
use selecting::select_columns;
use graph::{build_graph_from_csv, analyze_graph};
use std::fs;

// Validates if a file is non-empty
fn validate_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if fs::metadata(file_path)?.len() == 0 {
        return Err(format!("File {} is empty", file_path).into());
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // File paths for intermediate and final outputs
    let cleaned_file = "filtered_nfl.csv"; // Cleaned dataset
    let brady_file = "tom_brady_pass_attempts.csv"; // Filtered dataset for Tom Brady
    let final_file = "brady_filtered_columns.csv"; // Dataset with only desired columns

    // Step 1: Clean the original dataset
    // - Input: Raw NFL play-by-play dataset (nfl_pbp_2020_to_2022.csv)
    // - Output: Cleaned dataset (filtered_nfl.csv) with missing fields filled and rows standardized
    clean_csv("nfl_pbp_2020_to_2022.csv", cleaned_file)
        .map_err(|e| format!("Error cleaning CSV: {}", e))?;
    validate_file(cleaned_file)?; // Ensure the cleaned file is non-empty

    // Step 2: Filter for Tom Brady's passing plays
    // - Input: Cleaned dataset (filtered_nfl.csv)
    // - Output: Dataset containing only rows where Tom Brady is the passer and the play is a pass (tom_brady_pass_attempts.csv)
    filter_tom_brady(cleaned_file, brady_file)
        .map_err(|e| format!("Error filtering for Tom Brady's plays: {}", e))?;
    validate_file(brady_file)?; // Ensure the filtered file is non-empty

    // Step 3: Select desired columns
    // - Input: Tom Brady's passing plays dataset (tom_brady_pass_attempts.csv)
    // - Output: Dataset with only essential fields (brady_filtered_columns.csv)
    //   Fields: game_id, play_id, down, ydstogo, play_type, passer, receiver, yards_gained
    select_columns(brady_file, final_file)
        .map_err(|e| format!("Error selecting desired columns: {}", e))?;
    validate_file(final_file)?; // Ensure the final dataset is non-empty

    println!("Filtered CSV with desired columns created: {}", final_file);

    // Step 4: Build and analyze the graph
    // - Input: Final dataset (brady_filtered_columns.csv)
    // - Graph nodes: Individual pass attempts and receivers
    // - Graph edges: Connection between a pass attempt and the targeted receiver
    let csv_path = "brady_filtered_columns.csv";

    // Build the graph from the final dataset
    // - Each pass attempt is connected to its targeted receiver.
    let graph = build_graph_from_csv(csv_path)?;

    // Analyze the graph:
    // - Print basic graph statistics (nodes, edges)
    // - Calculate receiver target counts
    // - Optionally export the graph for visualization (e.g., DOT format for Graphviz)
    analyze_graph(&graph);

    Ok(())
}
