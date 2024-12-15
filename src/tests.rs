use ds210finalproject::cleaning::clean_csv;
use ds210finalproject::filtering::{filter_tom_brady, select_columns};
use std::fs::{File, read_to_string};
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, read_to_string};
    use std::io::{Write, Cursor};
    use std::error::Error;

    #[test]
    fn test_clean_csv() {
        let input_data = "game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained\n\
                        2020_01_TB_NO,233.0,,10.0,pass,T.Brady,C.Godwin,29.0\n\
                        2020_01_TB_NO,287.0,2.0,,pass,T.Brady,,8.0\n";

        let input_file_path = "test_input.csv";
        let output_file_path = "test_output.csv";

        // Write the input data to a file
        let mut input_file = File::create(input_file_path).expect("Failed to create input file");
        input_file
            .write_all(input_data.as_bytes())
            .expect("Failed to write input file");

        // Call the clean_csv function
        clean_csv(input_file_path, output_file_path).expect("Failed to clean CSV");

        // Read and verify the output
        let output_data = read_to_string(output_file_path).expect("Failed to read output file");
        println!("Actual Output:\n{}", output_data);

        // Expected output
        let expected_output = "game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained\n\
                            2020_01_TB_NO,233.0,0,10.0,pass,T.Brady,C.Godwin,29.0\n\
                            2020_01_TB_NO,287.0,2.0,0,pass,T.Brady,0,8.0\n";
        println!("Expected Output:\n{}", expected_output);

        // Compare line-by-line
        for (i, (actual, expected)) in output_data.lines().zip(expected_output.lines()).enumerate() {
            if actual != expected {
                println!("Mismatch on line {}:", i + 1);
                println!("Actual: {}", actual);
                println!("Expected: {}", expected);
            }
        }
        assert_eq!(output_data, expected_output);
    }
    #[test]
    fn test_filter_tom_brady() {
        let input_data = "game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained\n\
                        2020_01_TB_NO,233.0,1.0,10.0,pass,T.Brady,C.Godwin,29.0\n\
                        2020_01_TB_NO,287.0,2.0,10.0,run,T.Brady,,8.0\n\
                        2020_01_TB_NO,677.0,3.0,9.0,pass,J.Brees,S.Miller,8.0\n";

        let input_file_path = "test_filter_input.csv";
        let output_file_path = "test_filter_output.csv";

        // Write the input data to a file
        let mut input_file = File::create(input_file_path).expect("Failed to create input file");
        input_file.write_all(input_data.as_bytes()).expect("Failed to write input file");

        // Call the filter_tom_brady function
        filter_tom_brady(input_file_path, output_file_path).expect("Failed to filter Tom Brady's plays");

        // Read and verify the output
        let output_data = read_to_string(output_file_path).expect("Failed to read output file");

        // Expected output
        let expected_output = "game_id,play_id,down,ydstogo,play_type,passer,receiver,yards_gained\n\
                            2020_01_TB_NO,233.0,1.0,10.0,pass,T.Brady,C.Godwin,29.0\n";

        assert_eq!(output_data, expected_output);
    }
}

