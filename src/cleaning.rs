use csv::{ReaderBuilder, WriterBuilder};

pub fn clean_csv(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().from_path(input_file)?;
    let mut wtr = WriterBuilder::new().from_path(output_file)?;

    let headers = rdr.headers()?.clone();
    wtr.write_record(&headers)?;

    let mut processed_rows = 0;
    let mut skipped_rows = 0;

    for (line_number, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                let cleaned_row: Vec<String> = record.iter().enumerate().map(|(idx, value)| {
                    if value.trim().is_empty() {
                        if idx < 2 { // Example: Replace numeric fields with "0"
                            "0".to_string()
                        } else { // Replace text fields with empty string
                            "".to_string()
                        }
                    } else {
                        value.to_string()
                    }
                }).collect();

                // Skip rows with unequal lengths
                if cleaned_row.len() != headers.len() {
                    eprintln!(
                        "Skipping row {} due to unequal length (expected: {}, got: {})",
                        line_number + 1,
                        headers.len(),
                        cleaned_row.len()
                    );
                    skipped_rows += 1;
                    continue;
                }

                wtr.write_record(&cleaned_row)?;
                processed_rows += 1;
            }
            Err(err) => {
                eprintln!("Skipping row {} due to error: {}", line_number + 1, err);
                skipped_rows += 1;
            }
        }
    }

    wtr.flush()?;
    println!(
        "Cleaning completed: {} rows processed, {} rows skipped.",
        processed_rows, skipped_rows
    );
    Ok(())
}
