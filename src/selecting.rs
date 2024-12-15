use csv::{ReaderBuilder, WriterBuilder};

pub fn select_columns(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().from_path(input_file)?;
    let mut wtr = WriterBuilder::new().from_path(output_file)?;

    // Specify the desired columns
    let desired_columns = vec![
        "game_id", "play_id", "down", "ydstogo", "play_type", "passer", "receiver", "yards_gained",
    ];

    // Get the original headers and match indices for desired columns
    let headers = rdr.headers()?.clone();
    let indices: Vec<usize> = desired_columns
        .iter()
        .filter_map(|col| headers.iter().position(|h| h == *col))
        .collect();

    // Write the desired headers to the new CSV
    let selected_headers: Vec<&str> = indices.iter().map(|&i| headers.get(i).unwrap()).collect();
    wtr.write_record(&selected_headers)?;

    // Process each row and write only the desired columns
    for (line_number, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                let selected_row: Vec<String> = indices
                    .iter()
                    .map(|&i| record.get(i).unwrap_or("").to_string())
                    .collect();
                wtr.write_record(&selected_row)?;
            }
            Err(err) => {
                eprintln!("Skipping row {} due to error: {}", line_number + 1, err);
            }
        }
    }

    wtr.flush()?;
    Ok(())
}