package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"path/filepath"
	"sort"
	"time"

	"github.com/go-gota/gota/dataframe"
	"github.com/go-gota/gota/series"
	"github.com/sajari/regression"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s <path_to_csv_file>", os.Args[0])
	}
	filePath := os.Args[1]

	// aggiunge l'estensione .csv se manca
	if filepath.Ext(filePath) != ".csv" {
		filePath += ".csv"
	}

	// legge il dataframe direttamente dal percorso fornito
	df := readCSV(filePath)

	// riempie i valori nulli
	filledDf := fillMissingValues(df)

	// prepara le feature e il target
	X, Y, err := dataframeToXY(filledDf)
	if err != nil {
		log.Fatalf("Failed to convert dataframe: %v", err)
	}

	// Leave-One-Out Cross-Validation
	start := time.Now()
	predictions := leaveOneOutCV(X, Y)
	elapsed := time.Since(start).Seconds()

	// calcola l'mcc
	mcc := computeMCC(Y, predictions)

	// stampa i risultati
	fmt.Println("---metrics---")
	fmt.Printf("Dataset: %s\n", filePath)
	fmt.Printf("MCC: %.6f\n", mcc)
	fmt.Printf("time LOOCV: %.4f seconds\n", elapsed)
}

// converte il csv in un gota dataframe
func readCSV(path string) dataframe.DataFrame {
	f, err := os.Open(path)
	if err != nil {
		log.Fatalf("Cannot open file %s: %v", path, err)
	}
	defer f.Close()
	return dataframe.ReadCSV(f)
}

// riempie i valori mancanti
func fillMissingValues(df dataframe.DataFrame) dataframe.DataFrame {
	names := df.Names()
	if len(names) == 0 {
		return df
	}
	targetName := names[len(names)-1]

	newDf := df.Copy()

	for _, colName := range names {
		col := newDf.Col(colName)
		if !col.HasNaN() {
			continue
		}
		var replacement float64
		if colName == targetName {
			replacement = medianIgnoreNaN(&col)
		} else {
			replacement = meanIgnoreNaN(&col)
		}

		vals := col.Float()
		for i, v := range vals {
			if math.IsNaN(v) {
				vals[i] = replacement
			}
		}

		s := series.Floats(vals)
		s.Name = colName
		newDf = newDf.Mutate(s)
	}
	return newDf
}

// calcola la media ignorando i nan
func meanIgnoreNaN(s *series.Series) float64 {
	vals := s.Float()
	sum := 0.0
	count := 0
	for _, v := range vals {
		if !math.IsNaN(v) {
			sum += v
			count++
		}
	}
	if count == 0 {
		return 0.0
	}
	return sum / float64(count)
}

// calcola la mediana ignorando i nan
func medianIgnoreNaN(s *series.Series) float64 {
	vals := s.Float()

	clean := make([]float64, 0, len(vals))
	for _, v := range vals {
		if !math.IsNaN(v) {
			clean = append(clean, v)
		}
	}
	if len(clean) == 0 {
		return 0.0
	}
	sort.Float64s(clean)
	n := len(clean)
	if n%2 == 1 {
		return clean[n/2]
	}

	return clean[n/2-1]
}

// converte il dataframe in target e sample
func dataframeToXY(df dataframe.DataFrame) ([][]float64, []float64, error) {
	if df.Nrow() == 0 {
		return nil, nil, fmt.Errorf("empty dataframe")
	}
	names := df.Names()
	if len(names) == 0 {
		return nil, nil, fmt.Errorf("no columns")
	}
	targetIndex := len(names) - 1
	nRows := df.Nrow()
	nFeatures := len(names) - 1

	X := make([][]float64, nRows)
	for i := range X {
		X[i] = make([]float64, nFeatures)
	}
	Y := make([]float64, nRows)

	targetCol := df.Col(names[targetIndex]).Float()
	copy(Y, targetCol)

	featIdx := 0
	for j, name := range names {
		if j == targetIndex {
			continue
		}
		col := df.Col(name).Float()
		for i := 0; i < nRows; i++ {
			X[i][featIdx] = col[i]
		}
		featIdx++
	}
	return X, Y, nil
}

// leave one out cross validation
func leaveOneOutCV(X [][]float64, Y []float64) []int {
	n := len(X)
	preds := make([]int, n)

	for i := 0; i < n; i++ {

		trainX := make([][]float64, 0, n-1)
		trainY := make([]float64, 0, n-1)
		for j := 0; j < n; j++ {
			if j == i {
				continue
			}
			trainX = append(trainX, X[j])
			trainY = append(trainY, Y[j])
		}

		r := new(regression.Regression)
		r.SetObserved("target")
		for k := 0; k < len(trainX[0]); k++ {
			r.SetVar(k, fmt.Sprintf("var%d", k))
		}
		for idx, sample := range trainX {
			r.Train(regression.DataPoint(trainY[idx], sample))
		}
		r.Run()

		pred, _ := r.Predict(X[i])
		if pred >= 0.5 {
			preds[i] = 1
		} else {
			preds[i] = 0
		}
	}
	return preds
}

// calcola l'mcc
func computeMCC(yTrue []float64, yPred []int) float64 {
	if len(yTrue) != len(yPred) {
		log.Fatal("MCC: length mismatch")
	}
	var tp, tn, fp, fn float64
	for i := 0; i < len(yTrue); i++ {
		trueVal := int(yTrue[i] + 0.5)
		predVal := yPred[i]
		switch {
		case trueVal == 1 && predVal == 1:
			tp++
		case trueVal == 0 && predVal == 0:
			tn++
		case trueVal == 0 && predVal == 1:
			fp++
		case trueVal == 1 && predVal == 0:
			fn++
		}
	}
	numerator := tp*tn - fp*fn
	denominator := math.Sqrt((tp + fp) * (tp + fn) * (tn + fp) * (tn + fn))
	if denominator == 0 {
		return 0.0
	}
	return numerator / denominator
}
