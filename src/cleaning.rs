use csv::{ReaderBuilder, WriterBuilder};

pub fn clean_csv(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Open the input CSV file
    let mut rdr = ReaderBuilder::new().flexible(true).from_path(input_file)?;

    // Open the output CSV file
    let mut wtr = WriterBuilder::new().from_path(output_file)?;

    // Read and write headers
    let headers = rdr.headers()?.clone();
    wtr.write_record(&headers)?;

    // Process each row
    for (line_number, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                
                // Ensure all rows have the same number of columns as the header
                let cleaned_row: Vec<String> = (0..headers.len())
                    .map(|i| record.get(i).unwrap_or("").trim().to_string())
                    .map(|value| if value.is_empty() { "0".to_string() } else { value })
                    .collect();

                wtr.write_record(&cleaned_row)?;
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
fn test_clean_csv_fill_blanks() {
    // Input CSV data with missing fields
    let csv_data = "\
game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained
2020_01_TB_NO,233,1,10,pass,T.Brady,C.Godwin,29
2020_01_TB_NO,234,2,10,run,T.Brady,,0
,235,3,10,pass,T.Brady,M.Evans,
2020_01_TB_NO,236,,10,pass,T.Brady,S.Miller,8";

    // Expected cleaned data
    let expected_cleaned_data = "\
game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained
2020_01_TB_NO,233,1,10,pass,T.Brady,C.Godwin,29
2020_01_TB_NO,234,2,10,run,T.Brady,0,0
0,235,3,10,pass,T.Brady,M.Evans,0
2020_01_TB_NO,236,0,10,pass,T.Brady,S.Miller,8";

    // Create temporary input and output CSV files
    let input_file = "test_clean_csv_input.csv";
    let output_file = "test_clean_csv_output.csv";
    std::fs::write(input_file, csv_data).unwrap();

    // Run the clean_csv function
    clean_csv(input_file, output_file).expect("clean_csv failed");

    // Read the output file
    let output_data = std::fs::read_to_string(output_file).unwrap();

    // Log actual and expected outputs
    println!("Actual Output:\n{}", output_data);
    println!("Expected Output:\n{}", expected_cleaned_data);

    // Compare output to the expected cleaned data
    assert_eq!(output_data.trim(), expected_cleaned_data.trim(), "Cleaned data does not match expected output");

    // Cleanup
    std::fs::remove_file(input_file).unwrap();
    std::fs::remove_file(output_file).unwrap();
}
