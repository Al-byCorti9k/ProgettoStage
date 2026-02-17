package machinelearning

import (
	"bytes"
	"fmt"
	"math"

	"github.com/go-gota/gota/dataframe"
	"github.com/sjwhitworth/golearn/base"

	//"github.com/sjwhitworth/golearn/evaluation"
	"github.com/sjwhitworth/golearn/linear_models"
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

func PerformManualLOOCV(instances base.FixedDataGrid, targetColumn string) (float64, error) {

	var prediction []base.FixedDataGrid

	// 1. Identificazione dell'attributo target
	attributes := instances.AllAttributes()
	var targetAttr base.Attribute
	found := false

	for _, attr := range attributes {
		if attr.GetName() == targetColumn {
			targetAttr = attr
			found = true
			break
		}
	}

	if !found {
		return 0, fmt.Errorf("colonna target '%s' non trovata nel dataset", targetColumn)
	}

	// 2. Reset e impostazione del Target Attribute
	for _, classAttr := range instances.AllClassAttributes() {
		instances.RemoveClassAttribute(classAttr)
	}
	instances.AddClassAttribute(targetAttr)

	// 3. Preparazione per il loop LOOCV
	numInstances, _ := instances.Size()

	//Inizia il k-folding vero e proprio
	for i := 0; i < numInstances; i++ {
		// Create the test set with the i-th instance
		testRows := []int{i}
		testView := base.NewInstancesViewFromVisible(instances, testRows, instances.AllAttributes())
		testData := base.NewDenseCopy(testView)

		// Create the training set with all instances except the i-th
		trainRows := make([]int, 0, numInstances-1)
		for j := 0; j < numInstances; j++ {
			if j != i {
				trainRows = append(trainRows, j)
			}
		}
		trainView := base.NewInstancesViewFromVisible(instances, trainRows, instances.AllAttributes())
		trainData := base.NewDenseCopy(trainView)

		// Inizializzazione del modello (Logistic Regression)
		lr, err := linear_models.NewLogisticRegression("l2", 1.0, 1e-6)
		if err != nil {
			return 0, err
		}

		// Addestramento
		err = lr.Fit(trainData)
		if err != nil {
			return 0, fmt.Errorf("errore nel training al fold %d: %v", i, err)
		}

		// Predizione
		predictions, err := lr.Predict(testData)
		if err != nil {
			return 0, fmt.Errorf("errore nella prediction al fold %d: %v", i, err)
		}
		prediction = append(prediction, predictions)
	}

	mcc, err := calculateMCC(prediction, instances)
	if err != nil {
		return 0, err
	}
	return mcc, nil
}

func calculateMCC(predictions []base.FixedDataGrid, originalData base.FixedDataGrid) (float64, error) {
	// 1. Inizializziamo i contatori per la matrice di confusione (Assumendo classificazione binaria)
	// Se hai più classi, dovresti usare evaluation.GetConfusionMatrix
	var tp, tn, fp, fn float64

	numInstances, _ := originalData.Size()

	for i := 0; i < numInstances; i++ {
		// Estraiamo il valore reale (Ground Truth)
		realClassVal := base.GetClass(originalData, i)

		// Estraiamo la predizione (il test set in LOOCV ha sempre dimensione 1, quindi riga 0)
		predictedClassVal := base.GetClass(predictions[i], 0)

		// Nota: base.GetClass restituisce una stringa (il valore della categoria)
		// Assumiamo che "1" o "positive" sia la classe positiva
		// Modifica i letterali in base alle tue etichette reali
		if predictedClassVal == "1" {
			if realClassVal == "1" {
				tp++
			} else {
				fp++
			}
		} else {
			if realClassVal == "1" {
				fn++
			} else {
				tn++
			}
		}
	}

	// 2. Calcolo del coefficiente MCC
	// Formula: (TP*TN - FP*FN) / sqrt((TP+FP)(TP+FN)(TN+FP)(TN+FN))
	numerator := (tp * tn) - (fp * fn)
	denominator := math.Sqrt((tp + fp) * (tp + fn) * (tn + fp) * (tn + fn))

	if denominator == 0 {
		return 0, nil
	}

	return numerator / denominator, nil
}
