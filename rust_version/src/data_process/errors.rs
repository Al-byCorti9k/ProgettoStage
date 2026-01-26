//gestione degli errori unificata

use polars::prelude::PolarsError;
use std::io;

use crate::data_process::data::DatasetError;

#[derive(Debug)]
pub enum AppError {
    Polars(PolarsError),
    Dataset(DatasetError),
    Io(io::Error),
    Linfa(linfa_logistic::error::Error),
}

impl From<PolarsError> for AppError {
    fn from(e: PolarsError) -> Self {
        AppError::Polars(e)
    }
}

impl From<DatasetError> for AppError {
    fn from(e: DatasetError) -> Self {
        AppError::Dataset(e)
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<linfa_logistic::error::Error> for AppError {
    fn from(e: linfa_logistic::error::Error) -> Self {
        AppError::Linfa(e)
    }
}
