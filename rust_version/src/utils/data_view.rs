// modulo che implementa la gestione delle istruzioni da linea di comando 
// e la stampa dei risultati a schermo

use clap::Parser;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    
    #[arg(short, long, default_value = "1")]
   pub  dataset: Vec<usize>,
}

