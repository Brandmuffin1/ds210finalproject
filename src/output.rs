use csv::Writer;

pub fn write_filtered_csv(output_file: &str, rows: Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path(output_file)?;

    for row in rows {
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    Ok(())
}
