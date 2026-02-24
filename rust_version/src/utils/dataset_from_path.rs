//modulo per la creazione del dataframe polars dal csv e la produzione del csv finale con i risultati.
use crate::data_process::errors::AppError;
use polars::frame::DataFrame;
use polars::prelude::CsvReadOptions;
use polars::prelude::SerReader;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::data_process::data::get_dataset_info;
//funzione che ottiene dall'ID del dataset il percorso
pub fn get_dataset_path(index: Option<usize>) -> Result<(PathBuf, &'static str), AppError> {
    //iniziamo con il prendere il path
    let starting_path = Path::new(".");
    //vogliamo prelevare un dataset dal percorso "data"
    let mut csv_path = starting_path.join("..").join("..").join("..").join("data");
    //nota che join si occupa di mettere il separatore corretto per l'OS
    //scegliamo il dataset
    let selected_csv = get_dataset_info(index)?.get_csv();

    csv_path.push(selected_csv);

    Ok((csv_path, selected_csv))
}
//funzione che accetta un percorso ad un csv e genera un dataframe polars da esso
pub fn generate_df(csv_path: PathBuf) -> Result<DataFrame, AppError> {
    let df = CsvReadOptions::default()
        .with_infer_schema_length(Some(500))
        //imposta i parametri default per la lettura del .csv
        .try_into_reader_with_file_path(Some(csv_path.into())) //into() converte &str -> PathBuf
        .unwrap()
        .finish() //effettua l'effettiva conversione
        .unwrap();
    Ok(df)
}
//struttura dati che rappresenta i risultati raccolti in ogni iterazione
pub struct ResultData {
    dataset: Vec<String>,
    os: Vec<String>,
    time_s: Vec<f64>,
    time_ms: Vec<f64>,
    target_column: Vec<String>,
    mcc: Vec<f32>,
    energy: Vec<f64>,
}

impl ResultData {
    //Costruttore che inizializza i vettori vuoti
    pub fn new() -> Self {
        Self {
            dataset: Vec::new(),
            os: Vec::new(),
            time_s: Vec::new(),
            time_ms: Vec::new(),
            target_column: Vec::new(),
            mcc: Vec::new(),
            energy: Vec::new(),
        }
    }

    //Metodo per inserire una riga completa di dati
    pub fn add_record(
        &mut self,
        dataset: &str,
        os: &str,
        time_s: f64,
        time_ms: f64,
        target: &str,
        mcc: f32,
        energy: f64,
    ) {
        self.dataset.push(dataset.to_string());
        self.os.push(os.to_string());
        self.time_s.push(time_s);
        self.time_ms.push(time_ms);
        self.target_column.push(target.to_string());
        self.mcc.push(mcc);
        self.energy.push(energy);
    }
    //metodo che scrive resultData in un file csv
    pub fn write_csv(&self) -> Result<(), AppError> {
        //iniziamo con il prendere il path
        let starting_path = Path::new(".");
        //vogliamo prelevare un dataset dal percorso "data"
        let mut csv_path = starting_path.join("..").join("..").join("results");

        //il nome del file viene formattato con la data
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S");

        //costruisco il nome del file
        let file_name = format!("experiment_rust_{}.csv", timestamp);

        csv_path.push(file_name);

        let mut file = File::create(csv_path)?;

        //Header
        writeln!(
            file,
            "Dataset,Operating system,Column selected,LOOCV's time execusion (s),LOOCV's time execution (ms),MCC,energy consumption (kWh), methodology"
        )?;

        let len = self.dataset.len();

        for i in 0..len {
            let method = match self.os[i].eq("windows") {
                true => "Intel's VTune Profiler",
                _ => "RAPL interface",
            };

            writeln!(
                file,
                "\"{}\",{},\"{}\",{},{},{},{},{}",
                self.dataset[i],
                self.os[i],
                self.target_column[i],
                self.time_s[i],
                self.time_ms[i],
                self.mcc[i],
                self.energy[i],
                method
            )?;
        }

        Ok(())
    }
//metodo che stampa i valori della stuct ResultData sul terminale. Stampa ogni riga, che si riferisce ad ogni iterazione del ciclo del main
    pub fn print_table(&self) {
        let len = self.dataset.len();
        if len == 0 {
            println!("There are no result data.");
            return;
        }

        //Intestazioni delle colonne (uguali al CSV)
        let headers = [
            "Dataset",
            "Operating system",
            "Column selected",
            "LOOCV's time execusion (s)",
            "LOOCV's time execution (ms)",
            "MCC",
            "energy consumption (kWh)",
            "methodology",
        ];
        //Inizializza un array per memorizzare la larghezza massima di ciascuna colonna.
        //Partiamo dalla lunghezza dell'intestazione stessa.
        let mut col_widths = [0; 8];
        for (i, &h) in headers.iter().enumerate() {
            col_widths[i] = h.len();
        }

        //Prima passata sui dati: calcola la larghezza effettiva necessaria per ogni colonna
        //considerando la rappresentazione testuale di tutti i valori.
        for i in 0..len {
            let method = match self.os[i].as_str() {
                "windows" => "Intel's VTune Profiler",
                _ => "RAPL interface",
            };
            //Lunghezze dei valori in questa riga (come stringhe)
            let vals = [
                self.dataset[i].len(),
                self.os[i].len(),
                self.target_column[i].len(),
                format!("{}", self.time_s[i]).len(),
                format!("{}", self.time_ms[i]).len(),
                format!("{}", self.mcc[i]).len(),
                format!("{}", self.energy[i]).len(),
                method.len(),
            ];
            //Aggiorna la larghezza massima per ogni colonna se il valore corrente è più largo
            for (j, &w) in vals.iter().enumerate() {
                if w > col_widths[j] {
                    col_widths[j] = w;
                }
            }
        }

        //Macro per stampare una cella allineata a sinistra con una larghezza fissa,
        //seguita da due spazi di separazione tra le colonne.
        //$width: larghezza del campo, $value: valore da stampare (deve implementare Display)
        macro_rules! print_cell {
            ($width:expr, $value:expr) => {
                print!("{:<width$}  ", $value, width = $width);
            };
        }

        //Stampa l'intestazione (riga con i nomi delle colonne)
        for (i, &h) in headers.iter().enumerate() {
            print_cell!(col_widths[i], h);
        }
        println!(); //nuova linea dopo l'intestazione

        //Stampa righe di dati
        for i in 0..len {
            //Determina la metodologia in base al sistema operativo
            let method = match self.os[i].as_str() {
                "windows" => "Intel's VTune Profiler",
                _ => "RAPL interface",
            };
            //Stampa ogni campo della riga utilizzando la macro print_cell!
            print_cell!(col_widths[0], self.dataset[i]);
            print_cell!(col_widths[1], self.os[i]);
            print_cell!(col_widths[2], self.target_column[i]);
            //I valori numerici vengono convertiti in stringa al volo tramite format!,
            //ma la macro print_cell! si aspetta un valore che implementi Display;
            //passando direttamente format!("{}", ...) si crea una String temporanea.
            print_cell!(col_widths[3], format!("{}", self.time_s[i]));
            print_cell!(col_widths[4], format!("{}", self.time_ms[i]));
            print_cell!(col_widths[5], format!("{}", self.mcc[i]));
            print_cell!(col_widths[6], format!("{}", self.energy[i]));
            print_cell!(col_widths[7], method);
            println!(); //nuova linea dopo ogni riga di dati
        }
    }
}
