//il programma implementa una prima versione in cui semplicemente leggiamo
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello
//di regressione lineare

use std::env;
use std::path::Path;

use crate::utils::data_view::Args;
use clap::Parser;

use ndarray::{Array1, ArrayView1, ArrayView2};
use polars::prelude::*;
use std::fs::File;

pub mod data_process;
pub mod machine_learning;
pub mod utils;

use crate::data_process::data::get_dataset_info;
use crate::data_process::errors::AppError;
use crate::data_process::preprocessing::{ColumnsTypeConvertion, FillNullPolars, ScalerEncoder};
use crate::machine_learning::validation::{get_mcc, leave_one_out_cross_validation};

fn main() -> Result<(), AppError> {
    configure_the_environment();

    let args = Args::parse();
    let index = args.dataset[0];

    //iniziamo con il prendere il path
    let starting_path = Path::new(".");
    //vogliamo prelevare un dataset dal percorso "data"
    let mut csv_path = starting_path.join("..").join("..").join("..").join("data");
    //nota che join si occupa di mettere il separatore corretto per l'OS
    //scegliamo il dataset
    let selected_csv = get_dataset_info(Some(index))?.get_csv();

    csv_path.push(selected_csv);

    //ottenuto il percorso, con polars creiamo il relativo dataframe
    let mut df = CsvReadOptions::default()
        .with_infer_schema_length(Some(500))
        //imposta i parametri default per la lettura del .csv
        .try_into_reader_with_file_path(Some(csv_path.into())) //into() converte &str -> PathBuf
        .unwrap()
        .finish() //effettua l'effettiva conversione
        .unwrap();

    println! {"prima della conversione, la tabella è così: \n {}", df.tail(Some(5)) };

    //questo passaggio converte tutti dati dell'ultima colonna in i32
    let target_index = df.shape().1 - 1;

    //creo un iteratore, poi mappo su ogni elemento la chiusura che converte in stringa, dopo converto nella collezione
    let mut sample_col_names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    //ottengo il nome della colonna di interesse. In questo caso l'ultima
    let target_name = sample_col_names.swap_remove(target_index);

    df.sample_target_convertion(index, &target_name)?;

    df.cat_num_cols_to_fill()?;

    //dopo le conversioni, estraggo la colonna target dal dataframe originale
    
    let target_cols: Vec<i32> = df
        .column(&target_name)?
        .i32()?
        .into_no_null_iter()
        .collect();
    
    df.drop_in_place(&target_name)?;

    let mut sample_cols = df.scaler_encoder_df(index, &target_name)?;

    println!("inizio conversione in csv");
    let mut file = File::create("example.csv").expect("could not create file");
    CsvWriter::new(&mut file).finish(&mut sample_cols)?;
    println!("fine conversione in csv");
    //convertiamo in array2

    let sample_cols = sample_cols
        .to_ndarray::<Float64Type>(IndexOrder::Fortran)
        .unwrap();

    let sample_cols = ArrayView2::from(&sample_cols);

    //covertiamo in array1
    let target_col = Array1::from(target_cols);

    let target_col = ArrayView1::from(&target_col);

    let (original, prediction) = leave_one_out_cross_validation(sample_cols, target_col)?;

    let original = ArrayView1::from(&original);
    let prediction = ArrayView1::from(&prediction);
    let mcc = get_mcc(original, prediction)?;
    println!("il valore di mcc del dataset è: {}", mcc);

    Ok(())
}

// Configure Polars with ENV vars
// Rust richiede di usare unsafe Rust per la configurazione delle variabili
// d'ambiente. Queste servono per personalizzare l'aspetto delle tabelle
// Polars
pub fn configure_the_environment() {
    unsafe {
        env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1"); // mette gli angoli stondati
        env::set_var("POLARS_FMT_MAX_COLS", "20"); // per settare il numero massimo di colonne mostrate
        env::set_var("POLARS_FMT_MAX_ROWS", "10"); // stesso ma per le righe
        env::set_var("POLARS_FMT_STR_LEN", "50"); // numero massimo di caratteri per stringhe stampati
    }
}
