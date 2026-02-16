package machinelearning

import (
	"bytes"
	//"fmt"
	"github.com/go-gota/gota/dataframe"
	"github.com/sjwhitworth/golearn/base"
	//"github.com/sjwhitworth/golearn/evaluation"
	//"github.com/sjwhitworth/golearn/linear_models"
)

// ConvertGotaToGolearn trasforma un DataFrame gota in Instances di GoLearn
func ConvertGotaToGolearn(df dataframe.DataFrame) (base.FixedDataGrid, error) {
	// 1. Creiamo un buffer in memoria per scrivere il CSV
	buf := new(bytes.Buffer)

	// 2. Scriviamo il dataframe nel buffer in formato CSV
	err := df.WriteCSV(buf)
	if err != nil {
		return nil, err
	}

	reader := bytes.NewReader(buf.Bytes())
	// oltre a quella standard già presente nel CSV
	instances, err := base.ParseCSVToInstancesFromReader(reader, true)
	if err != nil {
		return nil, err
	}

	return instances, nil
}

/*
// Esegue la validazione Leave-One-Out utilizzando la Regressione Logistica
func performLOOCV(instances base.FixedDataGrid, targetColumnName string) (float64, error) {
	// 1. Identifica e imposta la colonna target tramite nome
	attributes := instances.AllAttributes()
	var targetAttr base.Attribute
	found := false

	for _, attr := range attributes {
		if attr.GetName() == targetColumnName {
			targetAttr = attr
			found = true
			break
		}
	}

	if !found {
		return 0, fmt.Errorf("colonna target '%s' non trovata nel dataset", targetColumnName)
	}

	// Comunica a GoLearn qual è la colonna di classe (target)
	instances.AddClassAttribute(targetAttr)

	// 2. Generazione dei Fold (LOOCV = numero di fold pari al numero di righe)
	numInstances, _ := instances.Size()
	folds, err := evaluation.GenerateCrossValidatedFolds(instances, numInstances)
	if err != nil {
		return 0, err
	}

	var totalAccuracy float64

	for _, fold := range folds {
		// 3. Inizializzazione e Training
		// Parametri: regolarizzazione "l2", coefficiente 1.0, tolleranza 1e-6
		lr, err := linear_models.NewLogisticRegression("l2", 1.0, 1e-6)
		if err != nil {
			return 0, err
		}

		err = lr.Fit(fold.TrainingData)
		if err != nil {
			return 0, err
		}

		// 4. Predizione e Valutazione
		predictions, err := lr.Predict(fold.TestData)
		if err != nil {
			return 0, err
		}

		confusionMat, _ := evaluation.GetConfusionMatrix(fold.TestData, predictions)
		totalAccuracy += evaluation.GetAccuracy(confusionMat)
	}

	// Ritorna la media dell'accuratezza
	return (totalAccuracy / float64(numInstances)) * 100, nil
}*/
