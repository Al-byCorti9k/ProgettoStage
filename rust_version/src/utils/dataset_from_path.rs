use crate::data_process::errors::AppError;
use polars::frame::DataFrame;
use polars::prelude::CsvReadOptions;
use polars::prelude::SerReader;
use std::path::Path;
use std::path::PathBuf;

use crate::data_process::data::get_dataset_info;

pub fn get_dataset_path(index: Option<usize>) -> Result<(PathBuf, &'static str), AppError> {
    //iniziamo con il prendere il path
    let starting_path = Path::new(".");
    //vogliamo prelevare un dataset dal percorso "data"
    let mut csv_path = starting_path.join("..").join("..").join("..").join("data");
    //nota che join si occupa di mettere il separatore corretto per l'OS
    //scegliamo il dataset
    let selected_csv = get_dataset_info(index)?.get_csv();

    csv_path.push(selected_csv);

    Ok((csv_path, selected_csv))
}

pub fn generate_df(csv_path: PathBuf) -> Result<DataFrame, AppError> {
    let df = CsvReadOptions::default()
        .with_infer_schema_length(Some(500))
        //imposta i parametri default per la lettura del .csv
        .try_into_reader_with_file_path(Some(csv_path.into())) //into() converte &str -> PathBuf
        .unwrap()
        .finish() //effettua l'effettiva conversione
        .unwrap();
    Ok(df)
}
