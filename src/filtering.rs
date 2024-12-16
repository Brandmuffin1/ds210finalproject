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

#[test]
fn test_filter_tom_brady() {
    // Input CSV data
    let csv_data = "\
game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained
2020_01_TB_NO,1,1,10,pass,T.Brady,A.Brown,15
2020_01_TB_NO,2,2,5,pass,T.Brady,M.Evans,8
2020_01_TB_NO,3,3,7,run,T.Brady,M.Evans,7
2020_01_TB_NO,4,1,10,pass,J.Doe,C.Godwin,20
";

    // Expected filtered data
    let expected_filtered_data = "\
game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained
2020_01_TB_NO,1,1,10,pass,T.Brady,A.Brown,15
2020_01_TB_NO,2,2,5,pass,T.Brady,M.Evans,8
";

    // Create temporary input and output CSV files
    let input_file = "test_filter_tom_brady_input.csv";
    let output_file = "test_filter_tom_brady_output.csv";
    std::fs::write(input_file, csv_data).unwrap();

    // Run the filter_tom_brady function
    filter_tom_brady(input_file, output_file).expect("filter_tom_brady failed");

    // Read the output file
    let output_data = std::fs::read_to_string(output_file).unwrap();

    // Compare output to the expected filtered data
    assert_eq!(output_data, expected_filtered_data, "Filtered data does not match expected output");

    // Cleanup
    std::fs::remove_file(input_file).unwrap();
    std::fs::remove_file(output_file).unwrap();
}
