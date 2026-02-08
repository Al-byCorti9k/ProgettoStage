//il programma implementa una prima versione in cui semplicemente leggiamo
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello
//di regressione lineare

use std::env;


use crate::utils::data_view::Args;
use crate::utils::dataset_from_path::{generate_df, get_dataset_path};
use clap::Parser;

use ndarray::{Array1, ArrayView1, ArrayView2};
use polars::prelude::*;
use std::fs::File;

pub mod data_process;
pub mod machine_learning;
pub mod utils;

use crate::data_process::errors::AppError;
use crate::data_process::preprocessing::{ColumnsTypeConvertion, FillNullPolars, ScalerEncoder};
use crate::machine_learning::validation::{get_mcc, leave_one_out_cross_validation};

fn main() -> Result<(), AppError> {
    //configura l'apparenza dei dataframe-polars
    configure_the_environment();
    //parsa gli argomenti da linea di comando
    let mut args = Args::parse();
    //effettua un controllo sugli argomenti
    args.argument_parse()?;
    
    //TODO inserire un ciclo che itera sugli indici di colonna scelti e le eventuali colonne target.

    let index = args.dataset.as_ref().and_then(| v| { Some(v[0]) });
    let target_name = args.target_columns.as_ref().and_then(|v| { Some(v[0].clone())});
    
    println!("{:?}", target_name);

    //otteniamo il dataframe polars dal percorso
    let mut df = generate_df(get_dataset_path(index)?)?;

    println! {"Selected dataset: \n {}", df.tail(Some(5)) };
    
    args.target_columns_check(&df, target_name.unwrap().as_str())?;

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
