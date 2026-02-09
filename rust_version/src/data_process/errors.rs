//gestione degli errori unificata

use polars::prelude::PolarsError;
use std::io;

use crate::{data_process::data::DatasetError, utils::data_view::ParserError};

#[derive(Debug)]
pub enum AppError {
    Polars(PolarsError),
    Dataset(DatasetError),
    Io(io::Error),
    LinfaLogistic(linfa_logistic::error::Error),
    Linfa(linfa::Error),
    Parser(ParserError),
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
        AppError::LinfaLogistic(e)
    }
}

impl From<linfa::Error> for AppError {
    fn from(e: linfa::Error) -> Self {
        AppError::Linfa(e)
    }
}

impl From<ParserError> for AppError {
    fn from(e: ParserError) -> Self {
        AppError::Parser(e)
    }
}
