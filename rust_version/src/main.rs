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
    let  mut df = CsvReadOptions::default() //imposta i parametri default per la lettura del .csv
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


    //l'obiettivo adesso è quello di creare una funzione che converta i dati in f64
    //qui ho avuto un problema perchè apply richeide un riferimento mutabile
    //mentre get_column_names() restituisce una stringa slice, che sappiamo essere di tipo &str (immutabile)
    //creo un iteratore, poi mappo su ogni elemento la chiusura che converte in stringa, dopo converto nella collezione
    let mut col_names : Vec<String> = df
                    .get_column_names()
                    .iter()
                    .map(| s | s.to_string())
                    .collect();
    //ottengo il nome della colonna di interesse. In questo caso l'ultima
    let target_name = col_names.swap_remove(target_index);

    for name in col_names {
       
        df.apply(&name, |s| {
            
            s.cast(&DataType::Float64).unwrap()
            
                
        })?;

    df.apply(&target_name, | s|{

            s.cast(&DataType::Int32).unwrap()
    })?;
    
    
}
    

    println!{"dopo la conversione la tabella è così:\n {}",df.tail(Some(5))};


    //TODO convertire la colonna target in array1, le altre in array2 per essere accettate da LINFA


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




