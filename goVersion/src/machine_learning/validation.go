package machinelearning

import (
	"encoding/csv"
	"fmt"
	"math"
	"os"
	"strconv"

	"gonum.org/v1/gonum/diff/fd"
	"gonum.org/v1/gonum/optimize"
)

func CSVToFloatMatrix(filename string) ([][]float64, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	reader := csv.NewReader(file)

	records, err := reader.ReadAll()
	if err != nil {
		return nil, err
	}
	// Ignora la prima riga (header)
	if len(records) > 0 {
		records = records[1:]
	}

	matrix := make([][]float64, len(records))

	for i, row := range records {
		matrix[i] = make([]float64, len(row))

		for j, value := range row {
			floatVal, err := strconv.ParseFloat(value, 64)
			if err != nil {
				return nil, fmt.Errorf("errore conversione riga %d colonna %d: %v", i, j, err)
			}
			matrix[i][j] = floatVal
		}
	}

	return matrix, nil
}

// Logistic calcola la funzione sigmoide: 1 / (1 + e^-z)

func sigmoid(z float64) float64 {

	return 1.0 / (1.0 + math.Exp(-z))

}

// LogisticRegression addestra il modello e restituisce i pesi (weights)
// x: matrice delle feature (input), y: etichette (0 o 1)
func LogisticRegression(x [][]float64, y []float64) []float64 {
	nFeatures := len(x[0])
	// Definiamo la funzione di costo (Negative Log-Likelihood)
	costFunc := func(w []float64) float64 {
		var loss float64
		for i := 0; i < len(x); i++ {
			// Calcolo del prodotto scalare (dot product) x_i * w
			var dot float64
			for j := 0; j < nFeatures; j++ {

				dot += x[i][j] * w[j]

			}
			h := sigmoid(dot)
			// Cross-Entropy Loss
			// Evitiamo log(0) aggiungendo un piccolo epsilon
			eps := 1e-15
			loss -= y[i]*math.Log(h+eps) + (1-y[i])*math.Log(1-h+eps)

		}
		return loss / float64(len(x))
	}

	// Punto di partenza (pesi inizializzati a zero)
	p := optimize.Problem{
		Func: costFunc,
		Grad: func(grad, w []float64) {
			fd.Gradient(grad, costFunc, w, nil)
		},
	}
	// Usiamo l'algoritmo BFGS (molto più veloce del Gradient Descent semplice)
	result, err := optimize.Minimize(p, make([]float64, nFeatures), nil, &optimize.BFGS{})
	if err != nil {
		fmt.Println("Errore nell'ottimizzazione:", err)
	}
	return result.X
}

// dato l'indice, estrae la colonna.
func RemoveColumn(matrix [][]float64, col int) ([][]float64, []float64, error) {
	if len(matrix) == 0 {
		return nil, nil, fmt.Errorf("matrice vuota")
	}

	if col < 0 || col >= len(matrix[0]) {
		return nil, nil, fmt.Errorf("indice colonna fuori range")
	}

	nRows := len(matrix)
	nCols := len(matrix[0])

	newMatrix := make([][]float64, nRows)
	removedColumn := make([]float64, nRows)

	for i := 0; i < nRows; i++ {

		if len(matrix[i]) != nCols {
			return nil, nil, fmt.Errorf("matrice non rettangolare alla riga %d", i)
		}

		// salva il valore della colonna rimossa
		removedColumn[i] = matrix[i][col]

		// crea nuova riga senza la colonna
		newRow := make([]float64, 0, nCols-1)
		//... sintassi espansione delle slices
		newRow = append(newRow, matrix[i][:col]...)
		newRow = append(newRow, matrix[i][col+1:]...)

		newMatrix[i] = newRow
	}

	return newMatrix, removedColumn, nil
}
