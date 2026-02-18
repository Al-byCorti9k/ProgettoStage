package machinelearning

import (
	"encoding/csv"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"

	//"gonum.org/v1/gonum/diff/fd"
	"gonum.org/v1/gonum/optimize"
)

func CSVToXY(filename string, targetColumn string) ([][]float64, []float64, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()
	reader := csv.NewReader(file)
	records, err := reader.ReadAll()
	if err != nil {
		return nil, nil, err
	}
	if len(records) < 2 {
		return nil, nil, fmt.Errorf("il CSV non contiene dati")
	}
	// Header
	header := records[0]
	// Trova indice colonna target
	targetIndex := -1
	for i, colName := range header {
		if colName == targetColumn {
			targetIndex = i
			break
		}
	}
	if targetIndex == -1 {
		return nil, nil, fmt.Errorf("colonna '%s' non trovata", targetColumn)
	}
	// Righe dati (salta header)
	data := records[1:]
	X := make([][]float64, len(data))
	Y := make([]float64, len(data))
	for i, row := range data {
		if len(row) != len(header) {
			return nil, nil, fmt.Errorf("numero colonne inconsistente alla riga %d", i+1)
		}
		Yval, err := strconv.ParseFloat(row[targetIndex], 64)
		if err != nil {
			return nil, nil, fmt.Errorf("errore conversione target riga %d: %v", i+1, err)
		}
		Y[i] = Yval
		// Costruisci riga X senza la colonna target
		newRow := make([]float64, 0, len(row)-1)
		for j, value := range row {
			if j == targetIndex {
				continue
			}
			floatVal, err := strconv.ParseFloat(value, 64)
			if err != nil {
				return nil, nil, fmt.Errorf("errore conversione riga %d colonna %d: %v", i+1, j, err)
			}
			newRow = append(newRow, floatVal)
		}
		X[i] = newRow
	}
	return X, Y, nil
}

// Logistic calcola la funzione sigmoide: 1 / (1 + e^-z)
func sigmoid(z float64) float64 {

	if z >= 0 {
		return 1.0 / (1.0 + math.Exp(-z))
	} else {
		return math.Exp(z) / (1.0 + math.Exp(-z))
	}
}

// LogisticRegression addestra il modello e restituisce i pesi (weights)
// x: matrice delle feature (input), y: etichette (0 o 1)
func LogisticRegression(x [][]float64, y []float64) []float64 {
	nFeatures := len(x[0])
	nSamples := float64(len(x))

	// Definiamo la funzione di costo (Negative Log-Likelihood)
	costFunc := func(w []float64) float64 {
		var loss float64
		for i := 0; i < len(x); i++ {
			var dot float64
			for j := 0; j < nFeatures; j++ {
				dot += x[i][j] * w[j]
			}
			h := sigmoid(dot)
			eps := 1e-15
			loss -= y[i]*math.Log(h+eps) + (1-y[i])*math.Log(1-h+eps)
		}
		return loss / nSamples
	}

	// Gradiente Analitico
	gradFunc := func(grad, w []float64) {
		// Azzera il gradiente per ogni chiamata
		for j := range grad {
			grad[j] = 0
		}

		for i := 0; i < len(x); i++ {
			var dot float64
			for j := 0; j < nFeatures; j++ {
				dot += x[i][j] * w[j]
			}

			// Errore: (predizione - valore reale)
			prediction := sigmoid(dot)
			diff := prediction - y[i]

			// Aggiorna il gradiente per ogni feature
			for j := 0; j < nFeatures; j++ {
				grad[j] += diff * x[i][j]
			}
		}

		// Normalizza per il numero di campioni
		for j := range grad {
			grad[j] /= nSamples
		}
	}

	p := optimize.Problem{
		Func: costFunc,
		Grad: gradFunc,
	}

	settings := optimize.Settings{
		MajorIterations: 1000,
	}

	result, err := optimize.Minimize(p, make([]float64, nFeatures), &settings, &optimize.BFGS{})
	if err != nil {
		log.Fatal(err)
	}

	return result.X
}

// Predict restituisce la probabilità che y = 1 per ogni esempio
func Predict(x [][]float64, weights []float64) []float64 {
	nSamples := len(x)
	probs := make([]float64, nSamples)
	for i := 0; i < nSamples; i++ {
		var dot float64
		for j := 0; j < len(weights); j++ {
			dot += x[i][j] * weights[j]
		}
		probs[i] = sigmoid(dot)
	}
	return probs
}

// PredictClass restituisce 0 o 1 usando una soglia
func PredictClass(x [][]float64, weights []float64) []int {
	probs := Predict(x, weights)
	classes := make([]int, len(probs))
	for i, p := range probs {
		if p >= 0.5 {
			classes[i] = 1
		} else {
			classes[i] = 0
		}
	}
	return classes
}

// MCC calcola il Matthews Correlation Coefficient
// yTrue: valori veri (0 o 1)
// yPred: valori predetti (0 o 1)
func MCC(yTrue []int, yPred []int) float64 {
	if len(yTrue) != len(yPred) {
		panic("Lunghezza di yTrue e yPred deve essere uguale")
	}

	var tp, tn, fp, fn float64

	for i := 0; i < len(yTrue); i++ {
		switch {
		case yTrue[i] == 1 && yPred[i] == 1:
			tp++
		case yTrue[i] == 0 && yPred[i] == 0:
			tn++
		case yTrue[i] == 0 && yPred[i] == 1:
			fp++
		case yTrue[i] == 1 && yPred[i] == 0:
			fn++
		}
	}

	num := tp*tn - fp*fn
	den := math.Sqrt((tp + fp) * (tp + fn) * (tn + fp) * (tn + fn))

	if den == 0 {
		return 0 // caso speciale, MCC non definito
	}

	return num / den
}

// usare il pacchetto gonum.org/v1/gonum/mat per gestire le matrici.
// LeaveOneOutCV esegue la Leave-One-Out cross-validation
// X: matrice delle feature
// y: etichette (0.0 o 1.0)
// ritorna slice di predizioni 0/1 e MCC
func LeaveOneOutCV(X [][]float64, y []float64) ([]int, float64) {
	nSamples := len(X)
	predictions := make([]int, nSamples)

	for i := 0; i < nSamples; i++ {
		// Creiamo X_train e y_train escludendo l'i-esimo esempio
		X_train := make([][]float64, 0, nSamples-1)
		y_train := make([]float64, 0, nSamples-1)
		for j := 0; j < nSamples; j++ {
			if j != i {
				X_train = append(X_train, X[j])
				y_train = append(y_train, y[j])
			}
		}

		// Addestriamo il modello sui n-1 esempi
		weights := LogisticRegression(X_train, y_train)

		// Prediciamo solo per l'esempio lasciato fuori
		prob := Predict([][]float64{X[i]}, weights)[0]

		// Convertiamo in 0 o 1 usando soglia 0.5
		if prob >= 0.5 {
			predictions[i] = 1
		} else {
			predictions[i] = 0
		}
	}

	// Convertiamo y in []int
	yInt := make([]int, nSamples)
	for i, v := range y {
		if v >= 0.5 {
			yInt[i] = 1
		} else {
			yInt[i] = 0
		}
	}

	// Calcoliamo MCC
	mcc := MCC(yInt, predictions)

	return predictions, mcc
}
