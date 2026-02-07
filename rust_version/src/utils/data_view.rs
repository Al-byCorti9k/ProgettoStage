// modulo che implementa la gestione delle istruzioni da linea di comando 
// e la stampa dei risultati a schermo

use clap::Parser;



#[derive(Parser, Debug)]
#[command(version, about = "\n A program to compute LOOCV, its execution time, and energy consumption (kWh).", long_about = None )]
pub struct Args {
    #[arg(short, long, help = "select target column from its name. ")]
   pub target_columns: Vec<String>, 
    #[arg(short, long, help = "select a dataset from its number", num_args(1..))]
   pub dataset: Option<Vec<usize>>,
}

