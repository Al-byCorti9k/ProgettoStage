//il programma implementa una prima versione in cui semplicemente leggiamo
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello
//di regressione lineare

use std::env;
use std::path::Path;


use polars::prelude::*;
//use ndarray::Array2;
use ndarray::{Array1, ArrayView1, ArrayView2};

pub mod data_process;
pub mod machine_learning;

use crate::data_process::data::get_dataset_info;
use crate::data_process::errors::AppError;

use crate::data_process::preprocessing::{ColumnsTypeConvertion, ModaInt};
use crate::machine_learning::validation::{get_mcc, leave_one_out_cross_validation};

fn main() -> Result<(), AppError> {
    configure_the_environment();
    //iniziamo con il prendere il path
    let starting_path = Path::new(".");
    //vogliamo prelevare un dataset dal percorso "data"
    let mut csv_path = starting_path.join("..").join("data");
    //nota che join si occupa di mettere il separatore corretto per l'OS
    //scegliamo il dataset
    let selected_cvs = "journal.pone.0175818_S1Dataset_Spain_cardiac_arrest_EDITED.csv";
    csv_path.push(selected_cvs);

    //ottenuto il percorso, con polars creiamo il relativo dataframe
    let mut df = CsvReadOptions::default()
        .with_infer_schema_length(Some(500))
        //imposta i parametri default per la lettura del .csv
        .try_into_reader_with_file_path(Some(csv_path.into())) //into() converte &str -> PathBuf
        .unwrap()
        .finish() //effettua l'effettiva conversione
        .unwrap();

    println! {"prima della conversione, la tabella è così: \n {}", df.tail(Some(5)) };

    //ottengo l'indice della colonna target
    let target_index = df.shape().1 - 1;

    //creo un iteratore, poi mappo su ogni elemento la chiusura che converte in stringa, dopo converto nella collezione

    let mut sample_col_names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    //ottengo il nome della colonna di interesse. In questo caso l'ultima
    let target_name = sample_col_names.remove(target_index);

    //converto i sample in f64, il target in i32
    df.sample_target_convertion(3, &target_name)?;
    println!("stampiamo dopo la conversione!\n {}", df.tail(Some(5)));
    //riempio i valori nulli della colonna target
    df.apply(&target_name, |c| {
        let mode =  c.i32().unwrap().calculate_mode().unwrap();
        c.i32().unwrap().fill_null_with_values(mode).unwrap().into_column()
    })?;
    //estraggo la colonna target e poi la elimino dal dataframe
    let target_cols: Vec<i32> = df
        .column(&target_name)?
        .i32()?
        .into_no_null_iter()
        .collect();

    df.drop_in_place(&target_name)?;
    println!("stampiamo dopo il drop \n {}", df.tail(Some(5)));

    //convertiamo in array2
    let sample_cols = df.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();

    let sample_cols = ArrayView2::from(&sample_cols);

    //covertiamo in array1
    let target_col = Array1::from(target_cols);

    let target_col = ArrayView1::from(&target_col);


    //cross validations
     let (original, prediction) = leave_one_out_cross_validation(sample_cols, target_col, &target_name, &sample_col_names)?;

    let original = ArrayView1::from(&original);
    let prediction = ArrayView1::from(&prediction);
    let mcc = get_mcc(original, prediction)?;


    // crea la matrice di confusione
    //let cm = prediction.confusion_matrix(&ground_truth).unwrap();

    println!("il valore di mcc è: {}", mcc);


    println! {"il dataset che ho selezionato è: {}\n", get_dataset_info(Some(3))?.get_csv() };

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


