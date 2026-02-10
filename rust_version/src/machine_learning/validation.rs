use crate::data_process::errors::AppError;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{ArrayView1, ArrayView2};
use rayon::prelude::*;
use std::time::Instant;

pub fn get_metrics<'a>(
    samples: ArrayView2<'a, f64>,
    target: ArrayView1<'a, i32>,
    energy_computation: bool,
) -> Result<Metrics, AppError> {
    let mut metrics = Metrics::new();

    let (original, prediction, time) = match energy_computation {
        true => {
            let start = Instant::now();

            let (original, prediction) = leave_one_out_cross_validation(samples, target)?;

            let elapsed = start.elapsed();

            (original, prediction, elapsed.as_secs_f64())
        }
        _ => {
            let start = Instant::now();

            let (original, prediction) = leave_one_out_cross_validation(samples, target)?;

            let elapsed = start.elapsed();

            (original, prediction, elapsed.as_secs_f64())
        }
    };

    //ottengo le corrispettive view dei risultati
    let original = ArrayView1::from(&original);
    let prediction = ArrayView1::from(&prediction);

    let mcc = get_mcc(original, prediction)?;
    metrics.set_mcc(mcc);
    metrics.set_time(time);
    metrics.set_energy(0.0);

    Ok(metrics)
}

//metodo pubblico che riceve in input i samples
// e il target nel formato ndarray e restituisce
//una reference
fn leave_one_out_cross_validation<'a>(
    samples: ArrayView2<'a, f64>,
    target: ArrayView1<'a, i32>,
) -> Result<(Vec<i32>, Vec<i32>), AppError> {
    let dataset = DatasetView::new(samples, target);
    //otteniamo il numero di campioni (righe)
    let n = dataset.nsamples();
    let style: ProgressStyle = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");
    // Trasformiamo le fold in un vettore per poter usare par_iter
    let folds: Vec<_> = dataset.fold(n).into_iter().collect();

    // Eseguiamo la computazione in parallelo grazie a Rayon
    let results: Result<Vec<(i32, i32)>, AppError> = folds
        //genera iteratori che lavorano in parallelo
        .into_par_iter()
        .progress_with_style(style)
        .map(|(train, valid)| {
            let model = LogisticRegression::default()
                .max_iterations(50)
                .with_intercept(true)
                .fit(&train)
                .map_err(AppError::from)?;

            let pred = model.predict(&valid);

            // Estraiamo il valore reale e quello predetto
            let true_val = valid.targets()[0];
            let pred_val = pred[0];

            Ok((true_val, pred_val))
        })
        .collect();

    // Trasformiamo il Vec<(i32, i32)> nelle due Vec richieste
    let (y_true, y_pred) = results?.into_iter().unzip();
    //Consuma un intero iteratore di coppie, producendo due collezioni: una composta dagli elementi a sinistra delle coppie e una dagli elementi a destra.
    Ok((y_true, y_pred))
}

//restituisce l'MCC. Ha bisogno di riceve in input i risultati della Leave One out folding.

fn get_mcc<'a>(y_true: ArrayView1<'a, i32>, y_pred: ArrayView1<'a, i32>) -> Result<f32, AppError> {
    let y_true: ndarray::ArrayBase<ndarray::OwnedRepr<usize>, ndarray::Dim<[usize; 1]>> =
        y_true.to_owned().mapv(|x| x as usize);
    let y_pred = y_pred.to_owned().mapv(|x| x as usize);

    // creaiamo la matrice di confusione
    let cm = y_pred.confusion_matrix(&y_true)?;

    //restituiamo l'mcc
    Ok(cm.mcc())
}
//struttura dati che contiene le metriche calcolate dal metodo get_metrics
#[derive(Debug, Clone)]
pub struct Metrics {
    pub mcc: f32,
    pub time: f64,
    pub energy: f64,
}
//metodi per inizializzare metrics e settare le metriche
impl Metrics {
    pub fn new() -> Self {
        Self {
            mcc: 0.0,
            time: 0.0,
            energy: 0.0,
        }
    }

    // --- Metodi Setter (Pattern Fluente) ---

    fn set_time(&mut self, time: f64) {
        self.time = time;
    }

    fn set_energy(&mut self, energy: f64) {
        self.energy = energy;
    }

    fn set_mcc(&mut self, mcc: f32) {
        self.mcc = mcc;
    }
}
