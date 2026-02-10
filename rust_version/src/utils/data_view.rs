// modulo che implementa la gestione delle istruzioni da linea di comando
// e la stampa dei risultati a schermo

use clap::{Parser, ArgAction};
use polars::{frame::DataFrame, prelude::BooleanChunked};

use crate::data_process::errors::AppError;

#[derive(Parser, Debug)]
#[command(version, about = " \n\n A program to compute LOOCV, its execution time, and energy consumption (kWh).", long_about = None )]
pub struct Args {
    #[arg(short, long, help = "select target column from its name.", num_args(1..))]
    pub target_columns: Option<Vec<String>>,
    #[arg(short, long, help = "select a dataset from its number", num_args(1..))]
    pub dataset: Option<Vec<usize>>,
    #[arg(
        short, 
        long, 
        default_value_t = true, 
        action = ArgAction::SetFalse,
        help = "Disable energy computation (defaults to true)"
    )]
    pub energy: bool,
    #[arg(
        short, 
        long, 
        action = ArgAction::SetTrue, 
        help = "get visual snapshots of selected datasets (defaults to false)"
    )]
    pub view: bool,
    #[arg(
        short, 
        long, 
        action = ArgAction::SetTrue, 
        help = "List of selectable datasets (defaults to false)"
    )]
    pub list: bool,
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
    DuplicatedDataset { message: &'static str },
    MoreTargetColumnsThanDataset { message: &'static str },
    MoreDatasetThanTargetColumns { message: &'static str },
    TargetColumnNotPresent { message: String },
    TargetColumnNotBiCat { message: String },
    TargetColumnNotBinary { message: String },
}

impl Args {
    //effettua i controlli sugli argomenti
    pub fn argument_parse(&mut self) -> Result<(), AppError> {
        let n1 = self.check_for_duplicates_columns()?;
        let n2 = self.check_for_duplicates_dataset()?;

        match n2 - n1 {
            //la destra dell'or è necessaria per mantenere la chiamata specifica
            //dei dataset senza senza dover obbligatoriamente inserire la t-column
            x if x == 0 || n1 == -1 => Ok(()),
            x if x > 0 => Err(AppError::Parser(
                ParserError::MoreDatasetThanTargetColumns {
                    message: "too many datasets (one per target column) ",
                },
            )),
            _ => Err(AppError::Parser(
                ParserError::MoreTargetColumnsThanDataset {
                    message: "too many target columns (one per dataset)",
                },
            )),
        }
    }
    //verifica che non ci siano doppioni nelle colonne di input
    //la verifica viene effettuata ordinando prima una copia del vec di input,
    //viene ordinata di modo che i doppioni siano consecutivi, e viene applicata
    //una funzione che elimina i doppioni consecutivi; se la lunghezza è variata
    //dopo il procedimento, allora vuol dire che c'erano dei doppioni. viene lanciato l'errore.
    fn check_for_duplicates_columns(&mut self) -> Result<i32, ParserError> {
        let col = self.target_columns.clone();
        let mut col_l: i32 = 0;
        let a: i32 = match col {
            Some(mut t) => {
                col_l = t.len().try_into().unwrap();
                t.sort();
                t.dedup();
                let col_l_d: i32 = t.len().try_into().unwrap();
                col_l - col_l_d
            }
            None => -1,
        };
        match a {
            0 => Ok(col_l),
            -1 => Ok(a),
            _ => Err(ParserError::DuplicatedColumns {
                message: "columns input should be different",
            }),
        }
    }

    //verifica che non ci siano doppioni nei dataset di input.
    //stessa logica di check_for_duplicates_columns
    fn check_for_duplicates_dataset(&mut self) -> Result<i32, ParserError> {
        let d_set = self.dataset.clone();
        let mut d_set_l: i32 = 0;
        let a: i32 = match d_set {
            Some(mut t) => {
                d_set_l = t.len().try_into().unwrap();
                t.sort();
                t.dedup();
                let d_set_l_d: i32 = t.len().try_into().unwrap();
                d_set_l - d_set_l_d
            }

            None => -1,
        };
        match a {
            0 => Ok(d_set_l),
            -1 => Ok(a),
            _ => Err(ParserError::DuplicatedDataset {
                message: "datasets input should be different",
            }),
        }
    }

    /*- verifica due condizioni:
    - che il nome appartenga al dataset selezionato corrispondente
    - se la prima è valida, le seguenti devono essere valide contemporaneamente:
        - la colonna deve avere solo due categorie
        - le due categorie devono corrispondere a 1 e 0.
    necessita che il dataframe polars sia già stato assemblato*/
    pub fn target_columns_check(
        &mut self,
        df: &DataFrame,
        target_name: &str,
    ) -> Result<(), AppError> {
        let column_contained = df.schema().get(target_name).is_some();

        match column_contained {
            true => self.target_column_is_bi_cat(df, target_name),
            _ => Err(AppError::Parser(ParserError::TargetColumnNotPresent {
                message: format!(
                    "target column ({}) should belongs to selected dataset",
                    target_name
                ),
            })),
        }
    }
    //verifica che la colonna scelta abbia due categorie
    fn target_column_is_bi_cat(
        &mut self,
        df: &DataFrame,
        target_name: &str,
    ) -> Result<(), AppError> {
        //calcola il numero di categorie
        let df_cat = df.group_by([target_name])?.groups()?;
        let n = df_cat[target_name].len();
        match n {
            2 => self.target_column_is_binary(&df_cat, target_name),
            _ => Err(AppError::Parser(ParserError::TargetColumnNotBiCat {
                message: format!(
                    "target column ({}) should contains only two categories",
                    target_name
                ),
            })),
        }
    }
    // verifica che le due categorie siano 0 e 1
    fn target_column_is_binary(
        &mut self,
        df_cat: &DataFrame,
        target_name: &str,
    ) -> Result<(), AppError> {
        let series = df_cat.column(target_name)?;
        let ca = series.i64()?;

        let mask: BooleanChunked = ca
            .into_iter()
            .map(|value_option| value_option.map(|value| value == 1 || value == 0))
            .collect();

        match mask.all() {
            true => Ok(()),

            _ => Err(AppError::Parser(ParserError::TargetColumnNotBinary {
                message: format!(
                    "target column ({}) should be binary for logistic regression's computation",
                    target_name
                ),
            })),
        }
    }
}
