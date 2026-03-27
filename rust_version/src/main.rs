use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::Array1;
use rayon::prelude::*;
use polars::prelude::*;
use std::env;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    // Leggi nome dataset da riga di comando
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <dataset.csv>", args[0]);
        std::process::exit(1);
    }
    let dataset_name = &args[1];
    
    
    // Usa il percorso fornito così com'è, aggiungendo .csv solo se non c'è estensione
    let mut path = PathBuf::from(dataset_name);
    if path.extension().is_none() {
        path.set_extension("csv");
    }
   

    // Carica CSV con Polars 
    let mut df = CsvReadOptions::default()
        .with_infer_schema_length(Some(500))
        .try_into_reader_with_file_path(Some(path.into()))
        .expect("reader error")
        .finish()
        .expect("csv reading error");
    

    //  Converti tutte le colonne in f64 
    let column_names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();

    for name in column_names {
        df.apply(&name, |s| s.cast(&DataType::Float64).unwrap())
            .unwrap();
    }
    //  Riempimento nulli 

    let column_names: Vec<String> = df
        .get_column_names()
        .iter()
        .map(|s| s.to_string())
        .collect();
    let n_cols = column_names.len();
    for (idx, name) in column_names.iter().enumerate() {
        let col = df.column(name).unwrap().clone();
        if col.null_count() == 0 {
            continue;
        }
        let fill_val = if idx == n_cols - 1 {
            col.f64().unwrap().median().unwrap()
        } else {
            col.f64().unwrap().mean().unwrap()
        };
        let filled = col.f64().unwrap().fill_null_with_values(fill_val).unwrap();
        df.replace_column(idx, filled).unwrap();
    }

    // Separa target (ultima colonna) e features
    let target_name = df.get_column_names().last().unwrap().to_owned();
    let target_col: Vec<f64> = df
        .column(&target_name)
        .unwrap()
        .f64()
        .unwrap()
        .into_no_null_iter()
        .collect();
    let mut features_df = df.clone();
    features_df.drop_in_place(&target_name).unwrap();
    let samples = features_df
        .to_ndarray::<Float64Type>(IndexOrder::Fortran)
        .unwrap();
    let target = Array1::from(target_col);

    // LOOCV con linfa 
    let dataset = DatasetView::new(samples.view(), target.view());
    let n = dataset.nsamples();
    let folds: Vec<_> = dataset.fold(n).into_iter().collect();

    let start = Instant::now();
    let results: Vec<(f64, f64)> = folds
        .into_par_iter()
        .map(|(train, valid)| {
            let model = LinearRegression::default()
                .fit(&train)
                .expect("Fit failed");
            let pred = model.predict(&valid);
            let true_val = valid.targets()[0];
            let pred_val = pred[0];
            (true_val, pred_val)
        })
        .collect();
    let elapsed = start.elapsed().as_secs_f64();

    //  Calcola MCC
    let (y_true, y_pred): (Vec<f64>, Vec<f64>) = results.into_iter().unzip();
    let y_true_usize: Array1<usize> = y_true.into_iter().map(|x| x as usize).collect();
    let y_pred_binary: Array1<usize> = y_pred
        .into_iter()
        .map(|x| if x > 0.5 { 1 } else { 0 })
        .collect();

    let cm = y_pred_binary
        .confusion_matrix(&y_true_usize)
        .expect("confusin matrix error");
    let mcc = cm.mcc();

    //  Stampa risultati 
    println!("---metrics---");
    println!("Dataset: {}", dataset_name);
    println!("MCC: {:.6}", mcc);
    println!("Time LOOCV: {:.6} seconds", elapsed);
}