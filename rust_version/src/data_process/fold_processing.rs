//modulo per la compatibilità dei datasetbase di linfa con
//i dataframe di polars

//INIZIO TEST PER COMPATIBILITA' DATASET-DATAFRAME
use linfa::{Dataset, DatasetView};
use ndarray::{Array2, ArrayView2, Axis, Ix1};
use polars::prelude::*;
use std::collections::HashMap;
use crate::data_process::{
    data::{VecToHash, get_dataset_info},
    errors::AppError,
    preprocessing::{FillNullPolars, ScalerEncoder, private::ScalersEncoders},
};

//definiamo la struct che rappresenta lo stato del processing di un fold
struct FitTrasformState {
    //salvo le medie delle colonne numeriche
    mean: HashMap<String, f64> ,
    //salvo la moda per le colonne categoriche
    mode: HashMap<String, i32> ,
    //salvo lo standard scaler
    std_scaler: HashMap<String, f64> ,
    //salvo la relazione nome colonna cat - vettore con le categorie nuove
    //per quella colonna. é necessario perchè in questo modo potrò verificare
    //se valid abbia un dato che non appartiene ad alcuna categoria
    map_category: Vec<Column>,
}

//creiamo delle funzioni utility per lo stato
impl FitTrasformState {
    //una funzione per il calcolo di media e moda per Train. Aggiorna lo stato
    //del fit trasform
    fn compute_median_mode_train(&mut self, dataframe: &mut DataFrame) -> Result<(), AppError> {
        let (mode, mean) = dataframe.cat_num_cols_to_fill()?;
        self.add_mean(mean);
        self.add_mode(mode);
        Ok(())
    }
    //una funzione che calcola la deviazione standard e la media di Train DOPO il riempimento dei valori. Può essere eseguita dopo il filling effettivo
    fn compute_std_encoding_train(
        &mut self,
        dataframe: &mut DataFrame,
        index: usize,
        target_column: &str,
    ) -> Result<DataFrame, AppError> {
        let cat_per_cols = dataframe.get_categories(index, target_column)?;

        let (df_encoded, std_scaler) = dataframe.scaler_encoder_df(index, target_column)?;

        self.add_std_scaler(std_scaler);
        self.add_map_category(cat_per_cols);

        Ok(df_encoded)
    }
    //una funzione che applica tutti i dati della struct al Valid per effettuare il preprocessing con gli stessi parametri che sono stati applicati per il train set
    fn trasform_mm_valid(&mut self, dataframe: &mut DataFrame) -> Result<(), AppError> {
        //riempio le celle vuote con la media
        for (k, v) in &self.mean{
            let col = dataframe.column(k.as_str())?.f64()?.fill_null_with_values(*v)?.into_series();
            dataframe.replace(k.as_str(), col)?;
        }
        //riempio le celle vuote con la moda
        for (k, v) in &self.mode{
            let col = dataframe.column(k.as_str())?.f64()?.fill_null_with_values(*v as f64)?.into_series();
            dataframe.replace(k.as_str(), col)?;
        }
        Ok(())
    }

    fn trasform_encoding_valid(&mut self, dataframe: &mut DataFrame, index: usize, target_column: &str) -> Result<DataFrame, AppError>{
        //TODO STD_SCALER E ONE HOT ENCODING
        let col_names = dataframe.get_column_names();
        let mut df = dataframe.clone();
        let mut df_new = DataFrame::default();
        let mut cat_cols_name = get_dataset_info(Some(index))?
            .get_cat_cols()
            .vec_to_hashset();
        cat_cols_name.remove(target_column);
         for col in col_names {
            if cat_cols_name.contains(col.as_str()) {
                df_new = df_new.hstack(df.to_dummies_valid(col.as_str(), self.map_category.remove(0))?.get_columns())?;
            } else {
                df.std_scaler_valid(col.as_str(), *self.std_scaler.get(&col.to_string()).unwrap())?;
            }
        }
        let finalized = df.finalization(&cat_cols_name, &df_new)?;
        
        Ok(finalized)
    }
    //metodo per creare un'istanza vuota di FitTrasformState
    fn new() -> Self {
        Self {
            mean: HashMap::new(),
            mode: HashMap::new(),
            std_scaler: HashMap::new(),
            map_category: Vec::new(),
        }
    }
    //metodo che aggiunge un elemento al vettore mean
    fn add_mean(&mut self, value:HashMap<String, f64> ) {
        self.mean = value;
    }
    //metodo che aggiunge un elemento al vettore moda
    fn add_mode(&mut self, value:HashMap<String, i32> ) {
        self.mode = value;
    }
    //metodo che aggiunge il vettore mean_std_scaler
    fn add_std_scaler(&mut self, value:HashMap<String, f64> ) {
        self.std_scaler = value;
    }
    //metodo che aggiunge il vettore colonne
    fn add_map_category(&mut self, value: Vec<Column>) {
        self.map_category = value;
    }

    //TODO: METODO PER SVUOTARE LA STRUCT DOPO OGNI CICLO/FOLD

    //TODO proseguire con il modello
}

//la funzione verrà chiamata per ogni fold e permetterà di usare i metodi per il
//preprocessing polars sui dataset linfa, attraverso varie conversioni
pub fn fold_dataset_preprocessing<'a>(
    train_dset: DatasetView<'a, f64, i32, Ix1>,
    valid_dset: DatasetView<'a, f64, i32, Ix1>,
    target_name: &str,
    sample_col_names: &Vec<String>,
) -> Result<(Dataset<f64, i32, Ix1>, Dataset<f64, i32, Ix1> ), AppError> {
   
//inizializzo la struct che rappresenta lo stato del preprocessing
let mut processing_state = FitTrasformState::new();
//estraggo dai dataset i sample e il target di train e valid
let sample_train = train_dset.records();
let target_train = train_dset.targets().to_owned();
let sample_valid = valid_dset.records();
let target_valid = valid_dset.targets().to_owned();
//convertiamo in un dataframe polars solo il sample del target
let mut df = ndarray_to_df(sample_train, sample_col_names)?;
//ora riempiamo i valori nulli. In ogni caso, la media e moda usata verranno salvati nello stato
processing_state.compute_median_mode_train(&mut df)?;
//ora effettuiamo la scalatura e il one hot encoding, salvando nello stato le relative informazioni
let df_new_train = processing_state.compute_std_encoding_train(&mut df, 3, target_name)?;
//ottenuta il sample train correttamente processato, usiamo la struct per processare con quei dati i sample valid

//convertiamo in un dataframe polars solo il sample del valid
let mut df_valid = ndarray_to_df(sample_valid, sample_col_names)?;
//riempio i possibili valori nulli con i dati della struct con moda o media
processing_state.trasform_mm_valid(&mut df_valid)?;
//effettuo su sample_valid il one-hot-encoding e la scalatura con i dati della struct
let df_new_valid = processing_state.trasform_encoding_valid(&mut df_valid, 3, target_name )?;

//ottenuti i due dataframe processati, vanno ricostruiti i dataset
//train
let df_new_train = df_new_train.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();
let sample_train = Array2::from(df_new_train);
let dataset_train = Dataset::new(sample_train, target_train);
//valid
let df_new_valid = df_new_valid.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();
let sample_valid = Array2::from(df_new_valid);
let dataset_valid = Dataset::new(sample_valid, target_valid );

Ok((dataset_train, dataset_valid))


}

//questa funzione restituisce un dataframe polars per effettuare il preprocessing
pub fn ndarray_to_df<'a>(
    arr: &ArrayView2<f64>,
    sample_col_names: &Vec<String>,
) -> Result<DataFrame, AppError> {
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
