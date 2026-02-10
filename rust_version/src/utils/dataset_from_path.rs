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
    // Costruttore che inizializza i vettori vuoti
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

    // Metodo per inserire una riga completa di dati
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

    pub fn write_csv(&self) -> Result<(), AppError> {
        //iniziamo con il prendere il path
        let starting_path = Path::new(".");
        //vogliamo prelevare un dataset dal percorso "data"
        let mut csv_path = starting_path.join("..").join("..").join("results");

        //il nome del file viene formattato con la data
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d_%H-%M-%S");

        // costruisco il nome del file
        let file_name = format!("experiment_rust_{}.csv", timestamp);

        csv_path.push(file_name);

        let mut file = File::create(csv_path)?;

        // Header
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
                "{},{},{},{},{},{},{},{}",
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
}
