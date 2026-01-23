//libreria di funzioni per il preprocessing


use polars::prelude::*;
use std::collections::HashMap;

//necessario per il calcolo della moda con valori che possono
//essere
use ordered_float::NotNan;

use crate::data_process::{
    data::{VecToHash, VecToHashSet, get_dataset_info},
    errors::AppError, preprocessing::private::ScalersEncoders,
};

//funzione per la conversione dei samples in f64 e del target in i32
//dato che to_dummies genera in automatico tabell2 i32, affido la responsabilità
//di convertire correttamente quelle celle a to_dummies64
pub trait ColumnsTypeConvertion {
    fn sample_target_convertion(
        &mut self,
        df_index: usize,
        target_name: &str,
    ) -> Result<(), AppError>;
}

impl ColumnsTypeConvertion for DataFrame {
    fn sample_target_convertion(
        &mut self,
        df_index: usize,
        target_name: &str,
    ) -> Result<(), AppError> {
        //ottengo l'Hashset dei nomi delle colonne categoriche
        let cat_cols = get_dataset_info(Some(df_index))?
            .get_cat_cols()
            .vec_to_hashset();
        //ottengo l'hashset con i nomi di tutte le colonne
        let binding = self.clone();
        let col_names = binding.get_column_names_str();
        let cols = col_names.vec_to_hashset();

        //itero sulla differenza tra i nomi di tutte le colonne e quelle categoriche, quindi quelle numeriche
        for col in cols.difference(&cat_cols) {
            self.apply(*col, |s| s.cast(&DataType::Float64).unwrap())?;
        }
        //Effettuo la conversione sulla colonna target.
        self.apply(target_name, |s| s.cast(&DataType::Int32).unwrap())?;

        Ok(())
    }
}

//abbiamo creato l'interfaccia per un metodo per ottenere dalla colonna
//il chunkedArray
pub trait ChunckedArrayFromColumn {
    fn get_chuncked_array_from_column_type(
        &self,
        column_type: &DataType,
    ) -> PolarsResult<NumericCA<'_>>;
}

//ho scritto due trait per il calcolo della moda.
//non mi piace molto perchè il codice è duplicato
//ciò che cambia è il tipo di ritorno e il tipo del parametro
pub trait ModaInt {
    type Output: ModeNumber;
    fn calculate_mode(&self) -> Option<Self::Output>;
}

pub trait ModaFloat {
    fn calculate_mode(&self) -> Option<f64>;
}

impl ModaInt for ChunkedArray<Int32Type> {
    type Output = i32;
    fn calculate_mode(&self) -> Option<Self::Output> {
        let mut occurrences = HashMap::new();

        for value in self.iter() {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by(|(a, ca), (b, cb)| {
                //controlla prima se i conteggi di a e b sono uguali
                //se sono uguali allora prende il minore tra le chiavi
                //quindi i valori
                ca.cmp(cb).then_with(|| b.cmp(a))
            })
            .map(|(val, _)| val)?
    }
}

impl ModaInt for ChunkedArray<Int64Type> {
    type Output = i64;
    fn calculate_mode(&self) -> Option<Self::Output> {
        let mut occurrences = HashMap::new();

        for value in self.iter() {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by(|(a, ca), (b, cb)| {
                //controlla prima se i conteggi di a e b sono uguali
                //se sono uguali allora prende il minore tra le chiavi
                //quindi i valori
                ca.cmp(cb).then_with(|| b.cmp(a))
            })
            .map(|(val, _)| val)?
    }
}
//cambia perchè è necessario l'uso di NotNan per compatibilità con le
//chiavi di hashMap
impl ModaFloat for ChunkedArray<Float64Type> {
    fn calculate_mode(&self) -> Option<f64> {
        let mut occurrences: HashMap<NotNan<f64>, i32> = HashMap::new();

        for value in self.into_no_null_iter() {
            let key = NotNan::new(value).ok()?;
            *occurrences.entry(key).or_insert(0) += 1;
        }
        //ora è necessario contare le occorrenze per determinare la moda
        //Questa versione tiene conto dello scenario di diversi valori
        //alternativi per la moda, e per questione di coerenza con
        //il progetto pandas, prendo il valore minore tra i due
        occurrences
            .into_iter()
            .max_by(|(a, ca), (b, cb)| {
                //controlla prima se i conteggi di a e b sono uguali
                //se sono uguali allora prende il minore tra le chiavi
                //quindi i valori
                ca.cmp(cb).then_with(|| b.cmp(a))
            })
            .map(|(val, _)| val.into_inner())
    }
}

//Enum serve per i tipi di conversione supportati
pub enum NumericCA<'a> {
    Int32(&'a ChunkedArray<Int32Type>),
    Float64(&'a ChunkedArray<Float64Type>),
    Int64(&'a ChunkedArray<Int64Type>),
}
pub trait ModeNumber {}
impl ModeNumber for i32 {}
impl ModeNumber for i64 {}

// implementazione per il type column di Polars
impl ChunckedArrayFromColumn for Column {
    fn get_chuncked_array_from_column_type(
        &self,
        column_type: &DataType,
    ) -> PolarsResult<NumericCA<'_>> {
        match column_type {
            DataType::Int32 => {
                let ca = self.i32()?;
                Ok(NumericCA::Int32(ca))
            }
            DataType::Float64 => {
                let ca = self.f64()?;
                Ok(NumericCA::Float64(ca))
            }
            DataType::Int64 => {
                let ca = self.i64()?;
                Ok(NumericCA::Int64(ca))
            }

            _ => panic!("type {} not supported", column_type),
        }
    }
}
//In questo modo abbiamo cat_num... accessibile in moduli esterni
//mentre i metodi usati da quest'ultimo sono privati
pub trait FillNullPolars: private::FillNullPolars {
    fn cat_num_cols_to_fill(&mut self) -> Result<(), PolarsError>;
}
//dentro a questo modulo ci metterò tutti i metodi con visibilità limitata a questo mosulo
pub(crate) mod private {

    use std::collections::HashSet;

    use polars::frame::DataFrame;

    use crate::data_process::errors::AppError;

    use super::{NumericCA, PolarsError};
    pub trait FillNullPolars {
        fn fill_dataframe_mode(
            &mut self,
            chuncked: NumericCA,
            idx: usize,
        ) -> Result<(), PolarsError>;

        fn fill_dataframe_median(
            &mut self,
            chuncked: NumericCA,
            idx: usize,
        ) -> Result<(), PolarsError>;
    }

    pub trait ScalersEncoders {
        //effettua lo standard scaler, sottraendo ad ogni elemento la media e dividendo per la deviazione std con ddof = 0
        fn std_scaler(&mut self, column_name: &str) -> Result<(), PolarsError>;

        //converte in automatico le colonne di to_dummies in f64
        fn to_dummies_f64(&mut self, column_name: &str) -> Result<DataFrame, PolarsError>;

        //per costruire il dataframe finale
        fn finalization(&mut self, hash_set: &HashSet<&str>, df: &DataFrame) -> Result<DataFrame, AppError>;
    }
}

//implementazioni per riempire con la moda i float e gli int. Un'altra funzione
//ha la responsabilità di verificare che siano categorici. Non ho trovato una
//soluzione migliore alla duplicazione. Sono tutti metodi "privati"
impl private::FillNullPolars for DataFrame {
    //implementazione per riempire i null con la moda
    fn fill_dataframe_mode(&mut self, chuncked: NumericCA, idx: usize) -> Result<(), PolarsError> {
        match chuncked {
            NumericCA::Int32(ca) => {
                let filled = ca.fill_null_with_values(ca.calculate_mode().unwrap())?;
                self.replace_column(idx, filled).unwrap();
                Ok(())
            }
            NumericCA::Int64(ca) => {
                let filled = ca.fill_null_with_values(ca.calculate_mode().unwrap())?;
                self.replace_column(idx, filled).unwrap();
                Ok(())
            }
            NumericCA::Float64(ca) => {
                let filled = ca.fill_null_with_values(ca.calculate_mode().unwrap())?;
                self.replace_column(idx, filled).unwrap();
                Ok(())
            }
        }
    }
    //implementazione per riempire i null con la media
    fn fill_dataframe_median(
        &mut self,
        chuncked: NumericCA,
        idx: usize,
    ) -> Result<(), PolarsError> {
        match chuncked {
            //fill_null con la media per i non categorici int
            NumericCA::Int32(ca) => {
                let series_f = ca.cast(&DataType::Float64)?;
                let ca_f = series_f.f64()?;
                let mean_value = ca_f.mean().unwrap();

                let filled = ca_f.fill_null_with_values(mean_value)?;
                // usa `filled` o sostituisci la colonna
                self.replace_column(idx, filled)?;
            }
            //per gli int64
            NumericCA::Int64(ca) => {
                let series_f = ca.cast(&DataType::Float64)?;
                let ca_f = series_f.f64()?;
                let mean_value = ca_f.mean().unwrap();

                let filled = ca_f.fill_null_with_values(mean_value)?;
                // usa `filled` o sostituisci la colonna
                self.replace_column(idx, filled)?;
            }
            //per i float
            NumericCA::Float64(ca) => {
                let mean_value = ca.mean().unwrap();

                let filled = ca.fill_null_with_values(mean_value)?;
                self.replace_column(idx, filled)?;
            }
        }

        Ok(())
    }
}
//metodo "pubblico" per la il riempimento delle celle vuote
impl FillNullPolars for DataFrame {
    //ritorna un vettore che contiene tutte le coppie indice e chucked array
    //da riempire
    fn cat_num_cols_to_fill(&mut self) -> Result<(), PolarsError> {
        //per usare le implementazioni private dei trait
        use crate::data_process::preprocessing::private::FillNullPolars as _;
        //ottengo le colonne categoriche
        let cat_cols = get_dataset_info(Some(3))
            .unwrap()
            .get_cat_cols()
            .vec_to_hashset();
        //ottengo il nome delle colonne categoriche
        let df_i = self.clone();
        let names: Vec<String> = df_i
            .get_column_names_str()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // enumerate è essenziale per ottenere l'indice della colonna
        for (idx, name) in names.into_iter().enumerate() {
            let s = df_i.column(&name)?;
            if s.null_count() != 0 && cat_cols.contains(name.as_str()) {
                let chuncked = s.get_chuncked_array_from_column_type(s.dtype())?;
                self.fill_dataframe_mode(chuncked, idx)?;
            } else if s.null_count() != 0 && !cat_cols.contains(name.as_str()) {
                let chuncked = s.get_chuncked_array_from_column_type(s.dtype())?;
                self.fill_dataframe_median(chuncked, idx)?;
            } else {
                continue;
            }
        }
        Ok(())
    }
}

pub trait ScalerEncoder: private::ScalersEncoders {

    fn scaler_encoder_df(&mut self, index: usize, target_column: &str) -> Result< DataFrame, AppError>;
}

impl ScalerEncoder for DataFrame {
    fn scaler_encoder_df(&mut self, index: usize, target_column: &str) -> Result< DataFrame, AppError> {
        //genero un dataframe vuoto iniziale
        let mut df = DataFrame::default();
        //ottengo il nome di tutte le colonne
        let binding = self.clone();
        let column_names = binding.get_column_names_str();
        //ottengo il nome delle colonne categoriche
        let mut cat_col_names = get_dataset_info(Some(index))?.get_cat_cols().vec_to_hashset();
        //elimino la colonna target dall'hashset
        cat_col_names.remove(target_column);

        for col_name in column_names{
            if cat_col_names.contains(col_name){
                 df = df.hstack(&self.to_dummies_f64(col_name)?.get_columns())?;
            }else {
                self.std_scaler(col_name)?;
            }
        }
        let finalized = self.finalization(&cat_col_names, &df)?;
        Ok(finalized)
        
    }
}

impl private::ScalersEncoders for DataFrame {
    //questo metodo si aspetta che i dati siano già convertiti in un tipo float
    //calcola lo standard scalar, ossia ogni numero di una colonna numerica
    //viene convertito di modo che abbia media 0 e std 1.
    fn std_scaler(&mut self, column_name: &str) -> Result<(), PolarsError> {
        //calcolo della deviazione standard della colonna. Lo "0" è un parametro che indica la deviazione standard della popolazione
        let col = self[column_name].as_materialized_series();
        //ddof deve essere 0 per coerenza con scikit-learn della controparte python
        let col_std = &col.std(0).unwrap();
        //calcolo della media della colonna
        let col_mean = &col.mean().unwrap();

        self.apply(column_name, |s| s - col_mean / col_std)?;

        Ok(())
    }

    fn to_dummies_f64(&mut self, column_name: &str) -> Result<DataFrame, PolarsError> {
        //genera dalla colonna selezionata n colonne binarie in base al numero di categorie presenti.
        let mut dummies =
            self[column_name]
                .as_materialized_series()
                .to_dummies(Some("_"), false, false)?;
        //creo una copia per accedere ai nomi
        let binding = dummies.clone();
        let names64 = binding.get_column_names_str();
        //conversione in f64
        for name in names64 {
            dummies.apply(&name, |s| s.cast(&DataType::Float64).unwrap())?;
        }
        //restituisce la colonna con
        Ok(dummies)
    }
    //costruisce il dataframe finale
    fn finalization(&mut self, hash_set: &std::collections::HashSet<&str>, df: &DataFrame) -> Result<DataFrame, AppError> {
        //dal dataframe originale elimina le colonne categoriche
        for key in hash_set{
            self.drop_in_place(key)?;
        }
        //concateno il le colonne non cat del dataframe originale
        //con le one-hot encoded 
        let finalized =self.hstack(df.get_columns())?;
        //restituisco un dataframe che rappresenta i samples
        Ok(finalized)

    }
    }

