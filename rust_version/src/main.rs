//il programma implementa una prima versione in cui semplicemente leggiamo 
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello 
//di regressione lineare

use std::path::Path;
use std::env;


use polars::prelude::*;
//use ndarray::Array1;

fn main() -> Result<(), PolarsError>{
    configure_the_environment();
    //iniziamo con il prendere il path
    let starting_path = Path::new(".");
    //vogliamo prelevare un dataset dal percorso "data"
    let mut csv_path = starting_path
                .join("..")
                .join("data");
    //nota che join si occupa di mettere il separatore corretto per l'OS
    //scegliamo il dataset
    let selected_cvs = "10_7717_peerj_5665_dataYM2018_neuroblastoma.csv";
    csv_path.push(selected_cvs);

    //ottenuto il percorso, con polars creiamo il relativo dataframe
    let  df = CsvReadOptions::default() //imposta i parametri default per la lettura del .csv
        .try_into_reader_with_file_path(Some(csv_path.into())) //into() converte &str -> PathBuf
        .unwrap() 
        .finish() //effettua l'effettiva conversione
        .unwrap();
    
    println!{"prima della conversione, la tabella è così: \n {}", df.tail(Some(5)) };

    //Conversione del dataframe di Polars in Array1 (target) e Array2(addestramento) per 
    //addestrare un modello di regressione logistica con "linfa"

    //conversione Array1
    
    //questo passaggio converte tutti dati dell'ultima colonna in i32
    let target_index = df.shape().1 - 1;
    let new_column_i32 = df.select_at_idx(target_index)
                                    .unwrap()
                                    .cast(&DataType::Int32)?; // <-- qui risolvi il Result
    //aggiungi la colonna con i tipi convertiti in i32 al dataframe mutabile originale
    //let df = df.with_column(new_column_i32)?;

    //ora faremo la stessa cosa ma per tutte le altre colonne, che devono essere convertite in f64
    //TODO scrivere la funzione apply che effettua la conversione di tutte le altre colonne


    //prelevo i sample per l'addestramento. Un riferimento mutevole alla slice di colonne
    let mut df_samples = df.select_by_range(0..target_index)?;

    //l'obiettivo adesso è quello di creare una funzione che converta i dati in f64
    //qui ho avuto un problema perchè apply richeide un riferimento mutabile
    //mentre get_column_names() restituisce una stringa slice, che sappiamo essere di tipo &str (immutabile)
    //creo un iteratore, poi mappo su ogni elemento la chiusura che converte in stringa, dopo converto nella collezione
    let col_names : Vec<String> = df_samples
                    .get_column_names()
                    .iter()
                    .map(| s | s.to_string())
                    .collect();

    for name in col_names {
       
        df_samples.apply(&name, |s| {
            
            s.cast(&DataType::Float64).unwrap()
            
                
        })?;
    
    
}
    let df = df_samples.insert_column(target_index, new_column_i32)?;

    println!{"dopo la conversione la tabella è così:\n {}",df.tail(Some(5))};


    //let y: Vec<i32> = new_column_i32.collect();

    //let y = Array1::from(y);
    
    Ok(())
    
}

// Configure Polars with ENV vars

pub fn configure_the_environment() {
    unsafe {
        env::set_var("POLARS_FMT_TABLE_ROUNDED_CORNERS", "1"); // mette gli angoli stondati
        env::set_var("POLARS_FMT_MAX_COLS", "20"); // per settare il numero massimo di colonne mostrate
        env::set_var("POLARS_FMT_MAX_ROWS", "10"); // stesso ma per le righe
        env::set_var("POLARS_FMT_STR_LEN", "50");  // numero massimo di caratteri per stringhe stampati
    }}

pub fn columns_convert_to_f64(){
    
}




