use std::fmt;
use std::any::Any;
use anyhow::{Result, anyhow};

use crate::series::Series;
use crate::data::Data;
use crate::types::{Int, Float, Text};

#[derive(Debug, Clone)]
pub enum Column {
    Int(Series<Int>),
    Float(Series<Float>),
    Text(Series<Text>),
}

impl Column {
    pub fn len(&self) -> usize {
        match self {
            Column::Int(series) => series.len(),
            Column::Float(series) => series.len(),
            Column::Text(series) => series.len(),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Column::Int(series) => series.is_empty(),
            Column::Float(series) => series.is_empty(),
            Column::Text(series) => series.is_empty(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Column::Int(series) => series.name(),
            Column::Float(series) => series.name(),
            Column::Text(series) => series.name(),
        }
    }
    pub fn add_value(&mut self, value: &dyn Any) -> Result<()> {
        match self {
            Column::Int(series) => {
                let value_t = value.downcast_ref::<Int>();
                if let Some(value_t) = value_t {
                    series.add_value(*value_t);
                } else {
                    return Err(anyhow!("Can't convert value to Int"));
                }
            },
            Column::Float(series) => {
                let value_t = value.downcast_ref::<Float>();
                if let Some(value_t) = value_t {
                    series.add_value(*value_t);
                } else {
                    return Err(anyhow!("Can't convert value to Float"));
                }
            },
            Column::Text(series) => {
                let value_t = value.downcast_ref::<Text>();
                if let Some(value_t) = value_t {
                    series.add_value(value_t.clone());
                } else {
                    return Err(anyhow!("Can't convert value to Text"));
                }
            },
        };
        Ok(())
    }
    pub fn into_int(self) -> Result<Self> {
        match self {
            Column::Int(series) => Ok(Column::Int(series)),
            Column::Float(_) => Err(anyhow!("Cannot cast float to int!")),
            Column::Text(series) => Ok(Column::Int(series.into_int()?)),
        }
    }
    pub fn into_float(self) -> Result<Self> {
        match self {
            Column::Int(series) => Ok(Column::Int(series)),
            Column::Float(_) => Err(anyhow!("Cannot cast float to int!")),
            Column::Text(series) => Ok(Column::Int(series.into_int()?)),
        }
    }
    pub fn into_text(self) -> Self {
        match self {
            Column::Int(series) => Column::Text(series.into_text()),
            Column::Float(series) => Column::Text(series.into_text()),
            Column::Text(series) => Column::Text(series),
        }
    }
    pub fn is_int(&self) -> bool {
        matches!(self, Column::Int(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self, Column::Float(_))
    }
    pub fn is_text(&self) -> bool {
        matches!(self, Column::Text(_))
    }
    pub fn data_any(&self) -> Vec<&dyn Data> {
        match self {
            Column::Int(series) => series.data_any(),
            Column::Float(series) => series.data_any(),
            Column::Text(series) => series.data_any(),
        }
    }
    pub fn data_box_any(&self) -> Vec<Box<dyn Data>> {
        match self {
            Column::Int(series) => series.data_box_any(),
            Column::Float(series) => series.data_box_any(),
            Column::Text(series) => series.data_box_any(),
        }
    }
    pub fn data_to_vector_string(&self) -> Vec<String> {
        match self {
            Column::Int(series) => series.data_to_vector_string(),
            Column::Float(series) => series.data_to_vector_string(),
            Column::Text(series) => series.data_to_vector_string(),
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Column::Int(series) => write!(f, "{}", series),
            Column::Float(series) => write!(f, "{}", series),
            Column::Text(series) => write!(f, "{}", series),
        }
    }
}
