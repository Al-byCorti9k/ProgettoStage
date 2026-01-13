//libreria di funzioni per il preprocessing

use polars::prelude::*;
use std::collections::HashMap;
//necessario per il calcolo della moda con valori che possono 
//essere
use ordered_float::NotNan;

//abbiamo creato l'interfaccia per un metodo per ottenere dalla colonna
//il chunkedArray
pub trait ChunckedArrayFromColumn {
    fn get_chuncked_array_from_column_type(&self,column_type: &DataType) ->
    PolarsResult<NumericCA<'_>> ;
}


//ho scritto due trait per il calcolo della moda.
//non mi piace molto perchè il codice è duplicato
//ciò che cambia è il tipo di ritorno e il tipo del parametro
pub trait ModaInt {
     fn calculate_mode(&self) -> Option<i32>;
}

pub trait ModaFloat {
    fn calculate_mode(&self) -> Option<f64>;
}

/* 
pub trait ModaModa {
    fn calculate_mode<T>(&self) -> Option<T>;
}

impl<T:PolarsDataType> ModaModa for ChunkedArray<T> {
    fn calculate_mode<D>(&self) -> Option<D> {
        let a = 8.4;
        Some(a)
    }
}*/

impl ModaInt for ChunkedArray<Int32Type>{

    fn calculate_mode(&self) -> Option<i32>{
        let mut occurrences = HashMap::new();

        for value in self.iter() {
             *occurrences.entry(value).or_insert(0) += 1;
            }

         occurrences.into_iter()
                       .max_by(|(a, ca), (b, cb)| {
                        //controlla prima se i conteggi di a e b sono uguali
                        //se sono uguali allora prende il minore tra le chiavi
                        //quindi i valori
                         ca.cmp(cb).then_with(|| b.cmp(a))
                                           })
                       .map(|(val, _)| val)?
    }
    
}

impl ModaFloat for ChunkedArray<Float64Type> {

    fn calculate_mode(&self) -> Option<f64> {
        let mut occurrences: HashMap<NotNan<f64>, i32>  = HashMap::new();

        for value in self.into_no_null_iter() {
             let key = NotNan::new(value).ok()?;
             *occurrences.entry(key).or_insert(0) += 1;
            }
    //ora è necessario contare le occorrenze per determinare la moda
    //Questa versione tiene conto dello scenario di diversi valori
    //alternativi per la moda, e per questione di coerenza con 
    //il progetto pandas, prendo il valore minore tra i due 
            occurrences.into_iter()
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
}
// implementazione per il type column di Polars
impl ChunckedArrayFromColumn for Column {

    fn get_chuncked_array_from_column_type(&self,column_type: &DataType) ->
    PolarsResult<NumericCA<'_>>  {

         match column_type {
                DataType::Int32 => {
                    let ca = self.i32()?;
                    Ok(NumericCA::Int32(ca))
                },
                DataType::Float64 => {
                    let ca = self.f64()?;
                    Ok(NumericCA::Float64(ca))

                },

                _ => panic!("type {} not supported", column_type)

                
            }}


    
}






    

