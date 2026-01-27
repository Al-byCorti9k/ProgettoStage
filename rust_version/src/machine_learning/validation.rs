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
    //let mut c = 0;
    //TODO test da effettuare. sembra vada in loop infinito
    for (train, valid) in dataset.fold(n) {
        let model = LogisticRegression::default()
            .max_iterations(50)
            .with_intercept(true)
            .fit(&train)?;

        let pred = model.predict(&valid);
        //target è un metodo di datasetView, ritorna un reference al field targets
        y_true.push(valid.targets()[0]);
        y_pred.push(pred[0]);

        // c += 1;
        // println!("fold {}", c);
    }
    Ok((y_true, y_pred))
}

//restituisce l'MCC. Ha bisogno di riceve in input i risultati della Leave One out folding.

pub fn get_mcc<'a>(
    y_true: ArrayView1<'a, i32>,
    y_pred: ArrayView1<'a, i32>,
) -> Result<f32, AppError> {
    let y_true: ndarray::ArrayBase<ndarray::OwnedRepr<usize>, ndarray::Dim<[usize; 1]>> =
        y_true.to_owned().mapv(|x| x as usize);
    let y_pred = y_pred.to_owned().mapv(|x| x as usize);

    /*
        use linfa::prelude::*;
    use ndarray::array;

    // create dummy classes 0 and 1
    let prediction: ndarray::ArrayBase<ndarray::OwnedRepr<usize>, ndarray::Dim<[usize; 1]>> = array![0, 1, 1, 1, 0, 0, 1];
    let ground_truth = array![0, 0, 1, 0, 1, 0, 1];
    */
    // creaiamo la matrice di confusione
    let cm = y_pred.confusion_matrix(&y_true)?;

    //restituiamo l'mcc
    Ok(cm.mcc())
}
