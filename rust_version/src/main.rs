//il programma implementa una prima versione in cui semplicemente leggiamo 
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello 
//di regressione lineare

use std::path::Path;
use std::env;


use polars::prelude::*;

//use ndarray::Array2;

pub mod data_process;

use crate::data_process::data::{VecToHash, get_dataset_info};
use crate::data_process::errors::AppError;

fn main() -> Result<(), AppError>{
    configure_the_environment();
    //iniziamo con il prendere il path
    let starting_path = Path::new(".");
    //vogliamo prelevare un dataset dal percorso "data"
    let mut csv_path = starting_path
                .join("..")
                .join("data");
    //nota che join si occupa di mettere il separatore corretto per l'OS
    //scegliamo il dataset
    let selected_cvs = "journal.pone.0175818_S1Dataset_Spain_cardiac_arrest_EDITED.csv";
    csv_path.push(selected_cvs);

    //ottenuto il percorso, con polars creiamo il relativo dataframe
    let  mut df = CsvReadOptions::default()
        .with_infer_schema_length( Some(500)) 
        //imposta i parametri default per la lettura del .csv
        .try_into_reader_with_file_path(Some(csv_path.into())) //into() converte &str -> PathBuf
        .unwrap() 
        .finish() //effettua l'effettiva conversione
        .unwrap();
    
    println!{"prima della conversione, la tabella è così: \n {}", df.tail(Some(5)) };
    let row = df.get_row(51)?;
    println!("{:?}", row);

    
    //let dtype = df.column("outcome")?.dtype();
    //println!("Il dtype è: {}\n", dtype);


    //Conversione del dataframe di Polars in Array1 (target) e Array2(addestramento) per 
    //addestrare un modello di regressione logistica con "linfa"

    //conversione Array1
    
    //questo passaggio converte tutti dati dell'ultima colonna in i32
    let target_index = df.shape().1 - 1;


    //l'obiettivo adesso è quello di creare una funzione che converta i dati in f64
    //qui ho avuto un problema perchè apply richeide un riferimento mutabile
    //mentre get_column_names() restituisce una stringa slice, che sappiamo essere di tipo &str (immutabile)
    //creo un iteratore, poi mappo su ogni elemento la chiusura che converte in stringa, dopo converto nella collezione
    let mut sample_col_names : Vec<String> = df
                    .get_column_names()
                    .iter()
                    .map(| s | s.to_string())
                    .collect();
    //ottengo il nome della colonna di interesse. In questo caso l'ultima
    let target_name = sample_col_names.swap_remove(target_index);

    // TODO chiamata alla funzione di preprocessing

    for name in sample_col_names {
       
        df.apply(&name, |s| {
            
            s.cast(&DataType::Float64).unwrap()
            
                
        })?;
    }
    df.apply(&target_name, | s|{

            s.cast(&DataType::Int32).unwrap()
    })?;
    
    

    //SEZIONE PER LA GESTIONE VALORI NULLI

    println!{"dopo la conversione la tabella è così:\n {}",df.tail(Some(5))};
    let row = df.get_row(51)?;
    println!("{:?}", row);

   let mut has_null  = df
    .null_count();        // produce un DataFrame
    
    println!("la colonna contiene valori nulli?: {}", has_null);
    
    // facciamo la trasposta del dataframe. L'idea è sfruttare 
    // l'ottimizzazione che Polars fa sulle operazioni per colonna
    // per velocizzare il controllo delle colonne con celle vuote
    let mut has_null_transpose = has_null.transpose(Some("columns"), None)?;

    has_null_transpose.rename("column_0", "null_count".into())?;
    // funzioni da me implementate per ottenere le colonne categoriche
    let cat_cols = get_dataset_info(Some(3))?.get_cat_cols().vec_to_hashset();
    //trasformo la colonna con il numero di elementi nulli per colonna in 
    // un chunckedArray Iterabile. 
    let s = has_null_transpose.column("null_count")?.u32()?; 
    // enumerate è essenziale per ottenere l'indice della colonna
    for (idx, opt_v) in s.into_iter().enumerate() {
    if let Some(v) = opt_v {
        if v > 0 {
            println!("questa colonna ha celle vuote");
            println!("con questo indice {}", idx);
            let name = df.get_column_names_str()[idx];
            println!("ecco la colonna {}", name);
            if cat_cols.contains(name) {
                println!("chiamo la funzione fill_null con moda");
                /* 
                let s_filled = df[name].fill_null(FillNullStrategy::Mean)?;
                df.replace(name, s_filled)?;
                */
                //l'idea è quella di fare match con dtype, con i tipi
                // i64  e f64. tutto questo vien fatto prima della conversione
                //finale!!
            }
            else {
                println!("chiamo la funzione fill_null con media");
            }
            
        }
        else {
            println!("questa cella non ha celle vuote");
        }
    }
}

//TODO usare questa versione
/*
    for name in df.get_column_names_str() {
    let s = df.column(name)?;
    if s.null_count() > 0 {
        df.apply(name, |s| Ok(s.fill_null(FillNullStrategy::Mean)?))?;
    }
}


*/



   // let names = has_null.get_column_names_str();



    // for name in names {

        //inserire logica
    //}

 

    

    
    



    

    //convertiamo in array2
    let df_linfa = df.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();
    println!{"dataframe convertito in ndarray2: \n {}", df_linfa};
    println!{"la riga numero 43: {:?}", df_linfa.row(41)};

    //quello che devi fare è una cosa molto diversa
    //TODO creare un iteratore sull'inteero dataframe per codificare correttamente i dati, distinguendo i categorici
    //dai non categorici
    //per questa parte puoi usare senza problemi gli iteratori
    //TODO quando devi usare linfa, usa to_ndarray, ma lo usi per convertire i dati; ad esempio, per i samples 
    //converti in array2<f64> per la colonna target in array2<i32> dalla quale poi estrai l'array array1<i32> che ti serve per linfa!

    //let y: Vec<i32> = new_column_i32.collect();

    //let y = Array1::from(y);

    println!{"il dataset che ho selezionato è: {}\n", get_dataset_info(Some(2))?.get_csv() };
   
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
        env::set_var("POLARS_FMT_STR_LEN", "50");  // numero massimo di caratteri per stringhe stampati
    }}





