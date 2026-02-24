package dataprocess

import (
	"fmt"

	"math"
	"sort" //per la mediana

	"github.com/go-gota/gota/series"
)

// funzione che gestice i valori vuoti di un dataframe
func FillColumnsNanValues(dfInfo *DataframeInfo) *DataframeInfo {

	df := fillNanWithMedian(dfInfo)

	newDfInfo := DataframeInfoBuild(dfInfo.Id, &df.Df)

	df2 := fillNanWithMean(&newDfInfo)

	finalDfInfo := DataframeInfoBuild(newDfInfo.Id, &df2.Df)

	return &finalDfInfo
}

// riempie i valori vuoti con la media
func fillNanWithMean(dfInfo *DataframeInfo) DataframeInfo {

	colsInfo, _ := GetDatasetInfo(&dfInfo.Id)
	catCols := colsInfo.VecToHashSet()
	//prendo tutte le colonne che non sono categoriche
	allCols := dfInfo.Df.Names()

	//Inizializziamo dfWorking con il dataframe originale
	dfWorking := dfInfo.Df
	//nel caso nessuna colonna avesse valori nulli
	i := 0

	for _, col := range allCols {
		colToCheck := dfWorking.Col(col)
		//per capire se sia categorica o meno
		_, exists := catCols[col]
		if colToCheck.HasNaN() && !exists {

			meanVal := calculateMean(&colToCheck)
			//modifica il dataframe originale con la nuova colonna
			dfWorking = dfWorking.Mutate(
				series.New(fillMissingValues(&colToCheck, meanVal), series.Float, col))
			i++
		}
	}
	if i == 0 {
		return *dfInfo
	}
	//alla fine ricostruisco un nuovo dataframe
	newDfInfo := DataframeInfoBuild(dfInfo.Id, &dfWorking)

	return newDfInfo
}

// calcola la media
func calculateMean(s *series.Series) float64 {
	if s.Len() == 0 {
		return 0.0
	}

	var sum float64
	var count int

	for i := 0; i < s.Len(); i++ {
		val := s.Elem(i).Float()

		// Controlliamo se il valore è un numero valido
		if !math.IsNaN(val) {
			sum += val
			count++
		}
	}

	// Se tutti i valori erano NaN, evitiamo la divisione per zero
	if count == 0 {
		return 0.0
	}

	return sum / float64(count)
}

// riempie le colonne categoriche con la moda
func fillNanWithMedian(dfInfo *DataframeInfo) DataframeInfo {

	cols, _ := GetDatasetInfo(&dfInfo.Id)
	catCols := cols.VecToHashSet()
	//Inizializziamo dfWorking con il dataframe originale
	dfWorking := dfInfo.Df
	//nel caso nessuna colonna avesse valori nulli
	i := 0

	for catCol, _ := range catCols {
		colToCheck := dfWorking.Col(catCol)
		if colToCheck.HasNaN() {
			modeVal := calculateMedian(&colToCheck)
			//modifica il dataframe originale con la nuova colonna
			dfWorking = dfWorking.Mutate(
				series.New(fillMissingValues(&colToCheck, modeVal), series.Float, catCol))
			i++
		}
	}
	if i == 0 {
		return *dfInfo
	}
	//alla fine ricostruisco un nuovo dataframe
	newDfInfo := DataframeInfoBuild(dfInfo.Id, &dfWorking)

	return newDfInfo
}

// Funzione di supporto per calcolare la moda in una Serie di Gota
//la lascio come alternativa possibile già testata
/*
func calculateMode(s *series.Series) float64 {
	counts := make(map[float64]int)
	maxFreq := 0
	var mode float64

	// Iteriamo sugli elementi della colonna
	for i := 0; i < s.Len(); i++ {
		val := s.Elem(i).Float()

		counts[val]++

		if counts[val] > maxFreq {
			maxFreq = counts[val]
			mode = val
		}
	}

	return mode
}
*/

// Funzione di supporto per calcolare la mediana inferiore in una Serie di Gota
func calculateMedian(s *series.Series) float64 {
	values := make([]float64, s.Len())

	//Copia gli elementi della serie in un array
	for i := 0; i < s.Len(); i++ {
		values[i] = s.Elem(i).Float()
	}

	//Ordina l'array
	sort.Float64s(values)

	//Calcola la mediana inferiore
	n := len(values)

	if n%2 == 1 {
		//Se il numero di elementi è dispari, la mediana è l'elemento centrale
		return values[n/2]
	} else {
		//Se il numero di elementi è pari, la mediana è l'elemento centrale inferiore
		return values[n/2-1]
	}
}

// riempie i valori nulli delle series con i valori che vuoi.
func fillMissingValues(s *series.Series, replacement float64) []float64 {
	//Estraiamo i valori come float64
	vals := s.Float()

	for i, v := range vals {
		//In Go, math.IsNaN è il modo standard per controllare i valori mancanti nei float
		if math.IsNaN(v) {
			vals[i] = replacement
		}
	}
	return vals
}

// si occupa di gestire il one-hot encoding su tutte le colonne categoriche del dataframe
func (self *DataframeInfo) OneHotEncoding(targetColumn string) *DataframeInfo {
	//ottengo l'hashset relativo alle colonne categoriche
	info, _ := GetDatasetInfo(&self.Id)

	catCols := info.VecToHashSet()
	delete(catCols, targetColumn)

	currentResult := *self
	//chiamo toDummies su tutte le colonne categoriche
	for cat := range catCols {
		currentResult = columnToDummies(&currentResult, cat)
	}
	//restituisco il dataframeInfo con il nuovo dataframe dove le colonne hanno
	//subito il one-hot-encoding
	return &currentResult
}

// implementazione custom del one-hot-encoding. Lavora solo su una colonna
func columnToDummies(dfInfo *DataframeInfo, targetColumn string) DataframeInfo {

	//ottengo la colonna come slice stringa e calcolo la lunghezza
	records := dfInfo.Df.Col(targetColumn).Records()
	numRows := len(records)

	// Otteniamo i valori univoci usando una mappa (set)
	uniqueMap := make(map[string]bool)
	//questa slice mantiene tutti i valori unici generati
	var uniqueValues []string
	//se un elemento non si trova nella mappa, viene aggiunto in uniqueMap e anche nella slice
	for _, val := range records {
		if !uniqueMap[val] {
			uniqueMap[val] = true
			uniqueValues = append(uniqueValues, val)
		}
	}
	dfWorking := dfInfo.Df

	//Generiamo le colonne One-Hot
	for _, val := range uniqueValues {
		// Creiamo una slice di float64 per la nuova colonna
		dummyData := make([]float64, numRows)
		//se è uguale alla categoria mette 1, altrimenti 0
		for i, rowVal := range records {
			if rowVal == val {
				dummyData[i] = 1.0
			} else {
				dummyData[i] = 0.0
			}
		}
		//generiamo il nuovo nome per la colonna
		colName := fmt.Sprintf("%s_%s", targetColumn, val)
		//generiamo la nuova colonna
		newCol := series.Floats(dummyData)

		//Impostiamo il nome alla nuova serie
		newCol.Name = colName
		//modifichiamo la colonna dal dataframe originale
		dfWorking = dfWorking.Mutate(newCol)
	}
	//eliminiamo la colonna originale (quella di input)
	finalDf := dfWorking.Drop(targetColumn)
	newDfInfo := DataframeInfoBuild(dfInfo.Id, &finalDf)

	return newDfInfo
}

// implementazione custom della scalatura standard
func (self *DataframeInfo) StandardScalar() *DataframeInfo {

	//Otteniamo i nomi delle colonne
	colsInfo, _ := GetDatasetInfo(&self.Id)
	catCols := colsInfo.VecToHashSet()
	//prendo tutte le colonne che non sono categoriche
	allCols := self.Df.Names()

	//Inizializziamo dfWorking con il dataframe originale
	dfWorking := self.Df

	for _, col := range allCols {
		colToCheck := dfWorking.Col(col)
		//per capire se sia categorica o meno
		_, exists := catCols[col]
		if !exists {

			mean := dfWorking.Col(col).Mean()
			std := dfWorking.Col(col).StdDev()

			if std == 0 {
				continue
			}

			//Estraiamo i valori della colonna convertendoli tutti in float64
			vals := colToCheck.Float()

			//Creiamo una nuova serie di tipo Float con la stessa lunghezza
			//e applichiamo la formula dello Z-score
			for i := 0; i < len(vals); i++ {
				vals[i] = (vals[i] - mean) / std
			}

			//Creiamo la nuova serie Gota forzando il tipo Float
			normalizedCol := series.Floats(vals)
			normalizedCol.Name = col //Mantieni il nome originale della colonna

			//Sostituiamo la colonna nel dataframe
			dfWorking = dfWorking.Mutate(normalizedCol)
		}

	}
	dfInfo := DataframeInfoBuild(self.Id, &dfWorking)
	return &dfInfo

}
