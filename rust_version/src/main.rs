//il programma implementa una prima versione in cui semplicemente leggiamo
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello
//di regressione lineare

use std::env;

use crate::utils::data_view::Args;
use crate::utils::dataset_from_path::{generate_df, get_dataset_path};
use clap::Parser;

use ndarray::{ArrayView1, ArrayView2};

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

    //cicla sul numero di dataset inseriti. Se non sono stati inseriti, avvia il default case.
    for i in 0..args.dataset.as_ref().map(|v| v.len()).unwrap_or(1) {
        //otteniamo l'indice del dataset selezionato da CLI
        let index = args.dataset.as_ref().and_then(|v| Some(v[i]));

        //otteniamo il target name selezionato da CLI
        let target_name = args
            .target_columns
            .as_ref()
            .and_then(|v| Some(v[i].clone()));

        //otteniamo il dataframe polars dal percorso
        let (path, dataset_name) = get_dataset_path(index)?;
        let mut df = generate_df(path)?;

        //visualizziamo il dataframe
        println! {"\n Selected dataset: \t {} \n {}", dataset_name, df.tail(Some(5)) };

        //estraiamo il nome. Se l'option è None, ritorna il nome dell'ultima colonna
        let target_name = df.unwrapping_column(target_name.as_deref());

        //effettua un controllo sulle colonne inserite. Un qualsiasi errore farà saltare il ciclo corrente
        if let Err(e) = args.target_columns_check(&df, target_name.as_str()) {
            eprintln!("\n Column '{}' ignored: \n {:?} \n", target_name, e);

            continue;
        }

        //conversione in i32 della colonna target e in f64 delle altre
        df.sample_target_convertion(index, &target_name)?;

        //riempimento dei valori nulli
        df.cat_num_cols_to_fill()?;

        //dopo le conversioni, estraggo la colonna target dal dataframe originale
        let target_cols: Vec<i32> = df
            .column(&target_name)?
            .i32()?
            .into_no_null_iter()
            .collect();

        //elimino la colonna target dal dataframe
        df.drop_in_place(&target_name)?;

        //scalatura e one-hot encoding
        let sample_cols = df.scaler_encoder_df(index, &target_name)?;

        //converto in ndarray1 e ndarray2
        let (sample_cols, target_col) = sample_cols.build_ndarrays(target_cols)?;

        //ottengo le corrispettive view
        let sample_cols = ArrayView2::from(&sample_cols);

        let target_col = ArrayView1::from(&target_col);

        //addestramento del modello, ottenimento dei valori di predizione per mcc
        ittapi::resume();
        let (original, prediction) = leave_one_out_cross_validation(sample_cols, target_col)?;

        //ottengo le corrispettive view dei risultati
        let original = ArrayView1::from(&original);

        let prediction = ArrayView1::from(&prediction);

        //calcolo mcc
        let mcc = get_mcc(original, prediction)?;
        println!("il valore di mcc del dataset è: {} \n", mcc);
    }
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
