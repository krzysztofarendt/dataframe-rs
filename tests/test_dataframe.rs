use anyhow::Result;

use dataframe::column::Column;
use dataframe::dataframe::DataFrame;
use dataframe::series::Series;
use dataframe::types::{Float, Int, Text};
use dataframe::config;

/// Checks if a dataframe saved to CSV is equal to dataframe loaded from CSV.
#[test]
fn create_write_and_read_csv() -> Result<()> {
    let mut df = DataFrame::new();

    let name = String::from("col_a");
    let data: Vec<Int> = vec![1, 2, 3];
    let series = Series::new(name, data);
    let column = Column::Int(series);
    df.add_column(column)?;

    let name = String::from("col_b");
    let data: Vec<Float> = vec![5.1234, 2.9999, 3.9];
    let series = Series::new(name, data);
    let column = Column::Float(series);
    df.add_column(column)?;

    let name = String::from("col_c");
    let data: Vec<Text> = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("!"),
    ];
    let series = Series::new(name, data);
    let column = Column::Text(series);
    df.add_column(column)?;

    // Save to CSV
    df.to_csv("test.csv", config::CSV_SEP)?;

    // Load from CSV
    let df_from_csv = DataFrame::from_csv("test.csv", config::CSV_SEP)?;

    // Check if both dataframes are equal
    assert_eq!(df, df_from_csv);    

    Ok(())
}
