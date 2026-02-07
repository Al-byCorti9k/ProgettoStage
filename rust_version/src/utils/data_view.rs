// modulo che implementa la gestione delle istruzioni da linea di comando 
// e la stampa dei risultati a schermo

use clap::Parser;

use crate::data_process::errors::AppError;



#[derive(Parser, Debug)]
#[command(version, about = "\n A program to compute LOOCV, its execution time, and energy consumption (kWh).", long_about = None )]
pub struct Args {
    #[arg(short, long, help = "select target column from its name.", num_args(1..))]
   pub target_columns: Option<Vec<String>>, 
    #[arg(short, long, help = "select a dataset from its number", num_args(1..))]
   pub dataset: Option<Vec<usize>>,
}
//metodo per la struct args che fa in modo che non ci siano argomenti ripetuti
//per dataset e column, inoltre se column.len != dataset.len, pareggia.
//se dataset > column, aggiunge dei None a column
//se column > dataset, toglie elementi a column per pareggiare.
//viene chiamata prima del ciclo.

// possibili errori di accesso ai dati
#[derive(Debug)]
pub enum ParserError {
    //se un utente seleziona un dataset fuori dal range, ottiene questo errore
    DuplicatedColumns { message: &'static str },
    DuplicatedDataset { message: &'static str},
    MoreTargetColumnsThanDataset {message: &'static str}
}

impl Args  {
    //effettua i controlli sugli argomenti
    pub fn argument_parse(&mut self) -> Result<(), AppError>{

        let n1 = self.check_for_duplicates_columns()?;
        let n2 = self.check_for_duplicates_dataset()?;

        match n2 - n1 {
        0 => Ok(()), 
        x if x > 0 => { self.target_columns.as_mut().unwrap().resize(n2.try_into().unwrap(), " ".to_string() );
        Ok(()) }
        _ => { Err(AppError::Parser(ParserError::MoreTargetColumnsThanDataset { message: "to many column targets" })) }
    }

        
    }
//verifica che non ci siano doppioni nelle colonne di input
    fn check_for_duplicates_columns(&mut self) -> Result<i32, ParserError>{

        let  col = self.target_columns.clone();
        
        let a: i32 = match col {

            Some(mut t) => {  
                            let col_l: i32 = t.len().try_into().unwrap();
                            t.sort();
                            t.dedup();
                            let col_l_d: i32 = t.len().try_into().unwrap();
                            col_l - col_l_d
                            },
            None => -1
            
        };
        match a  {
            0  => Ok(a),
            -1 => Ok(a), 
            _  => Err(ParserError::DuplicatedColumns { message: "try with different column next time" })
        }
}


//verifica che non ci siano doppioni nei dataset di input
     fn check_for_duplicates_dataset(&mut self) -> Result<i32, ParserError>{

        let d_set = self.dataset.clone();
        
        let a: i32 = match d_set {

            Some(mut t) => {  
                            let d_set_l: i32 = t.len().try_into().unwrap();
                            t.sort();
                            t.dedup();
                            let d_set_l_d: i32 = t.len().try_into().unwrap();
                            d_set_l - d_set_l_d
                            },


            None => -1
            
        };
        match a {
            0 => Ok(a),
            -1 => Ok(a),
            _ => Err(ParserError::DuplicatedColumns { message: "try with different datasets next time" })
        }
                                            }}

