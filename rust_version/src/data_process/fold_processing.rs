//modulo per la compatibilità dei datasetbase di linfa con
//i dataframe di polars

//INIZIO TEST PER COMPATIBILITA' DATASET-DATAFRAME
use linfa::{Dataset, DatasetView};
use ndarray::{Array2, ArrayView2, Axis, Ix1};
use polars::prelude::*;

use std::collections::HashMap;

use crate::data_process::{
    errors::AppError,
    preprocessing::{FillNullPolars, ScalerEncoder},
};

//definiamo la struct che rappresenta lo stato del processing di un fold
struct FitTrasformState {
    //salvo le medie delle colonne numeriche
    mean: Vec<f64>,
    //salvo la moda per le colonne categoriche
    mode: Vec<i32>,
    //salvo la media per lo standard scaler
    mean_std_scaler: Vec<f64>,
    //salvo la deviazione standard per colonna
    std_dev: Vec<f64>,
    //salvo la relazione numero colonna cat - vettore con le categorie nuove
    //per quella colonna. é necessario perchè in questo modo potrò verificare
    //se valid abbia un dato che non appartiene ad alcuna categoria
    map_category: HashMap<usize, Vec<String>>
}

//creiamo delle funzioni utility per lo stato
impl  FitTrasformState {
    
    //una funzione per il calcolo di media e moda per Train
    fn compute_median_mode_train(dataframe: &DataFrame) -> Result<(), AppError>{
        Ok(())
    }
    //una funzione che calcola la deviazione standard e la media di Train DOPO il riempimento dei valori
    fn compute_standard_scaler_train(dataframe: &DataFrame) -> Result<(), AppError>{
        Ok(())
    }
    //una funzione che usa i dati della struct per RIEMPIRE i dati mancanti in Train
    fn fill_train(dataframe: &mut DataFrame) -> Result<(), AppError>{
        Ok(())
    }
    //una funzione che 

//TODO proseguire con il modello

}





//la funzione verrà chiamata per ogni fold e permetterà di usare i metodi per il
//preprocessing polars sui dataset linfa, attraverso varie conversioni
pub fn fold_dataset_preprocessing<'a>(
    dataset: DatasetView<'a, f64, i32, Ix1>,
    target_name: &str,
    sample_col_names: &Vec<String>
) -> Result<Dataset<f64, i32, Ix1>, AppError> {
    //effettua l'accesso diretto ai sample
    let samples = dataset.records();
    let targets = dataset.targets().to_owned();
    //dai sample passo al dataframe polars
    let mut df = ndarray_to_df(samples, sample_col_names)?;
    println!("stampiamo il dataframe {}", df);
    //chiamo la funzione per il riempimento dei valori nulli
    println!("ciaooo prima del riempimento");
    df.cat_num_cols_to_fill()?;
    //chiamiamo la funzione per effettuare lo standard scaler e il one-hot-encoding
    println!("ciaooo prima dell'encoding!");
    let sample_processed = df.scaler_encoder_df(3, target_name)?;
    println!("ciaoo dopo l'encoding!");
    //ora riconvertiamo il dataframe in dataset
    let sample_processed = sample_processed
        .to_ndarray::<Float64Type>(IndexOrder::Fortran)
        .unwrap();
    let samples_p = Array2::from(sample_processed);

    let dataset = Dataset::new(samples_p, targets);
    Ok(dataset)
}







//questa funzione restituisce un dataframe polars per effettuare il preprocessing
pub fn ndarray_to_df<'a>(arr: &ArrayView2<f64>, sample_col_names: &Vec<String>) -> Result<DataFrame, AppError> {
    let mut columns = Vec::new();
    //prendiamo possesso dell'array
    let records = arr.to_owned();
    //generiamo le colonne a partire dall'array2D.
    for (i, col) in records.axis_iter(Axis(1)).enumerate() {
        let series = Column::new(sample_col_names[i].clone().into(), col.to_vec());
        //le colonne vengono pushate in un vettore
        columns.push(series);
    }
    //questa sezione serve per mantenere i null value che si perdono nella conversione
    let mut df_sample = DataFrame::new(columns)?;
    //ottengo il vettore dei nomi delle colonne samples
    let column_names: Vec<String> = df_sample
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    //itero sui nomi delle colonne
    for name in column_names {
        let s = df_sample.column(&name)?;
        //accetto il costo di clonare il nome della colonna
        let old_name = name.clone();
        //dato che il passaggio dataframe -> arraybase muta i null in FLoat(Nan)
        //per rimanere compatibile devo reinserire i null
        if matches!(s.dtype(), DataType::Float32 | DataType::Float64) {
            let new_s = s
                .f64()?
                .apply(|opt_v| opt_v.and_then(|v| if v.is_nan() { None } else { Some(v) }))
                .into_series()
                .rename(name.into())
                .to_owned();
            //rimpiazzo le colonne con quelle con i valori none
            df_sample.replace(&old_name, new_s)?;
        }
    }
    Ok(df_sample)
}
