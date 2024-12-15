use csv::{ReaderBuilder, WriterBuilder};

pub fn filter_tom_brady(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().flexible(true).from_path(input_file)?;
    let mut wtr = WriterBuilder::new().from_path(output_file)?;

    let headers = rdr.headers()?.clone();
    wtr.write_record(&headers)?;

    for (line_number, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                // Skip rows with unequal lengths
                if record.len() != headers.len() {
                    eprintln!(
                        "Skipping row {} due to unequal length (expected: {}, got: {})",
                        line_number + 1,
                        headers.len(),
                        record.len()
                    );
                    continue;
                }

                // Extract required columns
                let passer = record
                    .get(headers.iter().position(|h| h == "passer").unwrap())
                    .unwrap_or("");
                let play_type = record
                    .get(headers.iter().position(|h| h == "play_type").unwrap())
                    .unwrap_or("");

                // Filter rows where passer is T.Brady and play_type is pass
                if passer == "T.Brady" && play_type == "pass" {
                    wtr.write_record(&record)?;
                }
            }
            Err(err) => {
                eprintln!("Skipping row {} due to error: {}", line_number + 1, err);
            }
        }
    }

    wtr.flush()?;
    Ok(())
}