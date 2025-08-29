use anyhow::Result;

use dataframe::column::Column;
use dataframe::dataframe::DataFrame;
use dataframe::series::Series;
use dataframe::types::{Float, Int, Text};
use dataframe::config;
// use dataframe::data::Data;

fn main() -> Result<()> {
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

    println!("df:\n{}", df);

    df.to_csv("test.csv", config::CSV_SEP)?;
    let df = DataFrame::from_csv("test.csv", config::CSV_SEP)?;

    println!("df from csv:\n{}", df);

    // Some magic on columns
    // let column = df.get_column("col_a")?.as_any();
    // let column = column.downcast_ref::<Column>().unwrap().clone();

    // let column_data = df.get_column_data("col_a")?;
    // let data: Vec<Int> = column_data
    //     .iter()
    //     .map(|x| *(*x).as_any().downcast_ref::<Int>().unwrap()) // Copy occurs here
    //     .collect();
    // let column_data_copy = df.get_column_data_copy("col_a")?;

    // println!("column.is_int() = {:?}", data);

    Ok(())
}
