use linfa::metrics::ConfusionMatrix;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{ArrayView1, ArrayView2};

use crate::data_process::errors::AppError;

//metodo pubblico che riceve in input i samples
// e il target nel formato ndarray e restituisce
//una reference
pub fn leave_one_out_cross_validation<'a>(
    samples: ArrayView2<'a, f64>,
    target: ArrayView1<'a, i32>,
) -> Result<(Vec<i32>, Vec<i32>), AppError> {
    let dataset = DatasetView::new(samples, target);

    let n: usize = dataset.nsamples();

    let mut y_true: Vec<i32> = Vec::with_capacity(n);
    let mut y_pred: Vec<i32> = Vec::with_capacity(n);
    //effettuo il folding, cioè addestro con logistic regression con LOOCV
    println!("ciaoooo");
    //TODO test da effettuare. sembra vada in loop infinito
    for (train, valid) in dataset.fold(n) {
        let model = LogisticRegression::default()
            .max_iterations(1000)
            .with_intercept(true)
            .fit(&train)?;

        let pred = model.predict(&valid);
        //target è un metodo di datasetView, ritorna un reference al field targets
        y_true.push(valid.targets()[0]);
        y_pred.push(pred[0]);
    }
    println!("caio mondo");

    Ok((y_true, y_pred))
}

//restituisce l'MCC. Ha bisogno di riceve in input i risultati della Leave One out folding.

