use crate::data::Data;
use crate::types::{Float, Int, Text};
use anyhow::{Result, anyhow};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Series<T> {
    pub name: String,
    pub data: Vec<T>,
}

/// Public methods
impl<T> Series<T>
where
    T: 'static + Data + Clone + fmt::Display,
{
    pub fn new(name: String, data: Vec<T>) -> Self {
        Self { name, data }
    }
    pub fn add_value(&mut self, value: T) {
        self.data.push(value);
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn data_ref(&self) -> &Vec<T> {
        &self.data
    }
    pub fn data_any(&self) -> Vec<&dyn Data> {
        self.data.iter().map(|x| x as &dyn Data).collect()
    }
    /// Method to be used for pretty printing of columns and dataframes
    pub fn data_to_vector_string(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for x in self.data.iter() {
            v.push(x.to_string());
        }
        v
    }
    /// Returns a vector of type-erased elements.
    ///
    /// Example: Downcast back to i32
    /// let s = Series::new("numbers".to_string(), vec![1, 2, 3]);
    /// let anys = s.data_box_any();
    /// let first = anys[0].downcast_ref::<i32>().unwrap();
    /// assert_eq!(*first, 1);
    pub fn data_box_any(&self) -> Vec<Box<dyn Data>> {
        self.data
            .iter()
            .cloned()
            .map(|x| Box::new(x) as Box<dyn Data>)
            .collect()
    }
}

impl Series<Int> {
    pub fn into_float(self) -> Series<Float> {
        let mut data = Vec::new();
        for s in self.data {
            data.push(s as f64);
        }
        Series::new(self.name, data)
    }
    pub fn into_text(self) -> Series<Text> {
        let mut data = Vec::new();
        for s in self.data {
            data.push(format!("{}", s));
        }
        Series::new(self.name, data)
    }
}

impl Series<Float> {
    pub fn into_text(self) -> Series<Text> {
        let mut data = Vec::new();
        for s in self.data {
            data.push(format!("{}", s));
        }
        Series::new(self.name, data)
    }
}

impl Series<String> {
    pub fn into_int(&self) -> Result<Series<Int>> {
        let mut data = Vec::new();
        for s in &self.data {
            match s.parse::<i64>() {
                Ok(n) => data.push(n),
                Err(e) => return Err(anyhow!("Failed to parse '{s}' as i64: {e}")),
            }
        }
        Ok(Series::new(self.name.clone(), data))
    }
    pub fn into_float(&self) -> Result<Series<Float>> {
        let mut data = Vec::new();
        for s in &self.data {
            match s.parse::<f64>() {
                Ok(n) => data.push(n),
                Err(e) => return Err(anyhow!("Failed to parse '{s}' as f64: {e}")),
            }
        }
        Ok(Series::new(self.name.clone(), data))
    }
}

/// Implement Display trait
impl<T: fmt::Display> fmt::Display for Series<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Name of series
        write!(f, "{}: ", self.name)?;
        // Data
        for (i, x) in self.data.iter().enumerate() {
            write!(f, "{}", x)?;
            if i < self.data.len() - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}
