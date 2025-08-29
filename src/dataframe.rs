use crate::column::Column;
use crate::data::Data;
use crate::series::Series;
use crate::types::Text;
use crate::debug_print;
use crate::config;
use anyhow::{Result, anyhow};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct DataFrame {
    pub columns: Vec<Column>,  // TODO: Add HashMap <name, col_num>
    pub index: Option<String>, // TODO: This should be Index instance
    pub num_rows: usize,
    pub num_cols: usize,
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl DataFrame {
    /// Creates a new dataframe.
    pub fn new() -> Self {
        if config::DEBUG { debug_print!("Creating new DataFrame"); }
        Self {
            columns: Vec::new(),
            index: None,
            num_rows: 0,
            num_cols: 0,
        }
    }
    /// Adds a new column to the dataframe.
    pub fn add_column(&mut self, column: Column) -> Result<()> {
        if (self.num_cols > 0) && (column.len() != self.num_rows) {
            return Err(anyhow!(
                "New column has different length than existing columns"
            ));
        }
        for col in self.columns.iter() {
            if column.name() == col.name() {
                return Err(anyhow!(format!(
                    "Column name {} already exists",
                    column.name()
                )));
            }
        }
        self.num_rows = column.len();
        self.columns.push(column);
        self.num_cols += 1;
        Ok(())
    }
    pub fn get_column(&self, name: &str) -> Result<&Column> {
        let position = self.columns.iter().position(|x| x.name() == name);
        if let Some(position) = position {
            Ok(&self.columns[position])
        } else {
            Err(anyhow!("Column not found: {}", name))
        }
    }
    pub fn get_column_data(&self, name: &str) -> Result<Vec<&dyn Data>> {
        let position = self.columns.iter().position(|x| x.name() == name);
        if let Some(position) = position {
            Ok(self.columns[position].data_any())
        } else {
            Err(anyhow!("Column not found: {}", name))
        }
    }
    pub fn get_column_data_copy(&self, name: &str) -> Result<Vec<Box<dyn Data>>> {
        let position = self.columns.iter().position(|x| x.name() == name);
        if let Some(position) = position {
            Ok(self.columns[position].data_box_any())
        } else {
            Err(anyhow!("Column not found: {}", name))
        }
    }
    /// Saves dataframe to a CSV with a chosen separator string.
    pub fn to_csv(&self, path: &str, sep: &str) -> Result<()> {
        let mut file = File::create(path)?;
        let field_width = 0;
        file.write_all(self.pretty_string(sep, Some(field_width), None).as_bytes())?;
        Ok(())
    }
    /// Reads dataframe from a CSV with a chosen separator string.
    pub fn from_csv(path: &str, sep: &str) -> Result<Self> {
        // Read file to string
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Parse the string
        let lines: Vec<&str> = contents.lines().collect();
        let header: Vec<&str> = lines[0].split(sep).collect();

        // Initiallize dataframe
        let mut df = Self::new();
        let mut columns: Vec<Column> = Vec::new();

        // Read the header
        let col_names: Vec<String> = header.iter().map(|x| x.to_string()).collect();
        println!("col_names={:?}", col_names);

        // Initialize series as text (will cast to other types later)
        for col in col_names {
            let series = Series::new(col, Vec::<Text>::new());
            columns.push(Column::Text(series));
        }

        // Fill series with data (treat everything as text)
        // Skip the header, start from 1
        for line in lines.iter().skip(1) {
            let mut fields: Vec<String> = line.split(sep).map(|x| x.to_string()).collect();
            for col in columns.iter_mut().rev() {
                let any = fields.pop().unwrap();
                col.add_value(any.as_any())?;
            }
        }

        // Cast to correct types. Int -> Float -> Text
        let mut typed_columns: Vec<Column> = Vec::new();
        for col in &columns {
            // Try Int
            if let Column::Text(series) = col {
                if let Ok(s) = series.into_int() {
                    typed_columns.push(Column::Int(s));
                    continue;
                }
                if let Ok(s) = series.into_float() {
                    typed_columns.push(Column::Float(s));
                    continue;
                }
                // else
                typed_columns.push(col.clone());
            }
        }

        let columns = typed_columns;

        // Add columns do dataframe
        for series in columns.into_iter() {
            df.add_column(series)?;
        }

        Ok(df)
    }

    /// Sets index on a column.
    pub fn set_index(&mut self, name: &str) -> Result<()> {
        for col in self.columns.iter() {
            if col.name() == name {
                self.index = Some(name.to_string());
                return Ok(());
            }
        }
        Err(anyhow!("Column {} not found!", name))
    }
    /// Converts the DataFrame into a pretty string.
    ///
    /// `sep`         - value separator, e.g. ","
    /// `field_width` - width of the columns. Values are aligned to the right.
    /// `max_rows`    - if not None, defines how many rows to print
    pub fn pretty_string(
        &self,
        sep: &str,
        field_width: Option<usize>,
        max_rows: Option<usize>,
    ) -> String {
        // Get the field width (columns will be adjusted to the right)
        let width = field_width.unwrap_or(config::FIELD_WIDTH);
        // Get the num rows (only these many rows will be printed if it's not None)
        let num_rows = max_rows.unwrap_or(self.num_rows);
        let num_rows_skipped: usize = self.num_rows - num_rows;
        // Initialize output string
        let mut s = String::new();
        // Header
        for c in 0..self.num_cols {
            let col_name = &self.columns[c].name();
            s.push_str(&format!("{:>width$}", col_name));
            if c < self.num_cols - 1 {
                s.push_str(sep);
            } else {
                s.push('\n');
            }
        }
        // Data
        let mut col_strings: Vec<Vec<String>> = Vec::new();
        for c in 0..self.num_cols {
            let col = &self.columns[c];
            col_strings.push(col.data_to_vector_string());
        }

        for r in 0..num_rows {
            for (c, col) in col_strings.iter().enumerate() {
                s.push_str(&format!("{:>width$}", col[r]));
                if c < self.num_cols - 1 {
                    s.push_str(sep);
                } else {
                    s.push('\n');
                }
            }
        }
        if num_rows_skipped > 0 {
            s.push_str(&format!("... ({} rows skipped)\n", num_rows_skipped));
        }
        s
    }
}

impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_width = Some(config::FIELD_WIDTH);
        let max_rows = None;
        write!(f, "{}", self.pretty_string(config::CSV_SEP, field_width, max_rows))
    }
}
