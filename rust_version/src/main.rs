//il programma implementa una prima versione in cui semplicemente leggiamo
//un dataset con polars, convertiamo i dati correttamente, e poi addestriamo un modello
//di regressione lineare

use std::env;
use std::path::Path;

use linfa::prelude::*;
use polars::prelude::*;
//use ndarray::Array2;
use ndarray::{Array1, ArrayView1, ArrayView2, Axis};
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::any::type_name;
use std::fs::File;

pub mod data_process;
pub mod machine_learning;

use crate::data_process::data::get_dataset_info;
use crate::data_process::errors::AppError;
use crate::data_process::preprocessing::{ColumnsTypeConvertion, FillNullPolars, ScalerEncoder};
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

    //questo passaggio converte tutti dati dell'ultima colonna in i32
    let target_index = df.shape().1 - 1;

    //creo un iteratore, poi mappo su ogni elemento la chiusura che converte in stringa, dopo converto nella collezione
    
    let mut sample_col_names: Vec<String> = df.get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    //ottengo il nome della colonna di interesse. In questo caso l'ultima
    let target_name = sample_col_names.swap_remove(target_index);

    df.sample_target_convertion(3, &target_name)?;

    let samples = df.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();
    println!(
        "il dataframe convertito in ndarray con i valori nulli\n{:?}",
        samples
    );
    println!("dopo la conversione in ndarray, ecco cosa succede ai valori nulli:\n");
    let row = samples.row(51);
    println!("{:?}", row);
    let row2 = samples.row(233);
    println!("{:?}", row2);

    let targets = Array1::<i32>::zeros(422);
    //proviamo a creare un dataset linfa dall'ndarray trovato
    let dataset = Dataset::new(samples, targets);

    //proviamo a riconvertire in ndarray il dataset linfa
    let records = dataset.records().to_owned();

    //proviamo a passare dall'arraybase al dataframe polars
    let mut columns = Vec::new();

    for (i, col) in records.axis_iter(Axis(1)).enumerate() {
        let series = Column::new(format!("col_{}", i).into(), col.to_vec());
        columns.push(series);
    }
    //questa sezione serve per mantenere i null value che si perdono nella conversione
    let df69 = DataFrame::new(columns)?;
    let mut ruf = df69.clone();
    for name in df69.get_column_names() {
    let s = ruf.column(name)?;

    if matches!(s.dtype(), DataType::Float32 | DataType::Float64) {
        let new_s = s
            .f64()?
            .apply(|opt_v| opt_v.and_then(|v| if v.is_nan() { None } else { Some(v) }))
            .into_series()
            .rename(name.clone())
            .to_owned();

        ruf.replace(name, new_s)?;
    }
}



    println! {"stampiamo il dataframe riconvertito\n {}", df69.tail(Some(5))};

    //mi aspetto di ritrovarmi le righe ancora con i null
    let row = ruf.get_row(51)?;
    println!("{:?}", row);
    let row2 = ruf.get_row(233)?;
    println!("{:?}", row2);

    df.cat_num_cols_to_fill()?;

    println!("{:?}", df.shape());

    println! {"dopo la conversione la tabella è così:\n {}",df.tail(Some(5))};

    let row = df.get_row(51)?;
    println!("{:?}", row);
    let row2 = df.get_row(233)?;
    println!("{:?}", row2);

    //dopo le conversioni, estraggo la colonna target dal dataframe originale
    //let v1 = df.column(&target_name)?;
    let target_cols: Vec<i32> = df
        .column(&target_name)?
        .i32()?
        .into_no_null_iter()
        .collect();

    df.drop_in_place(&target_name)?;

    println!("stampiamo dopo il drop \n {}", df.tail(Some(5)));

    let sample_cols = df.scaler_encoder_df(3, &target_name)?;

    println! {"dopo one-hot-encoding e il resto è così: \n {}", sample_cols.tail(Some(5)) };

    //convertiamo in array2

    let sample_cols = sample_cols
        .to_ndarray::<Float64Type>(IndexOrder::Fortran)
        .unwrap();

    let sample_cols = ArrayView2::from(&sample_cols);

    //covertiamo in array1
    let target_col = Array1::from(target_cols);

    let target_col = ArrayView1::from(&target_col);

    //let (original, prediction) = leave_one_out_cross_validation(sample_cols, target_col)?;

    //let original = ArrayView1::from(&original);
    //let prediction = ArrayView1::from(&prediction);
    //let mcc = get_mcc(original, prediction)?;

    //SEZIONE PER IL CSV NON SERVE AL MOMENTO
    /*
    let s1 = Column::new("rust_pred".into(), prediction);
    let mut df4  = DataFrame::new(vec![s1])?;
    let mut file = File::create("example.csv").expect("could not create file");
    CsvWriter::new(&mut file)
    .finish(&mut df4)?;
    */

    // create confusion matrix
    // let cm = prediction.confusion_matrix(&ground_truth).unwrap();

    //println!("il valore di mcc è: {}", cm.mcc());

    //TODO Interazione Main con linfa per l'addestramento

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

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
