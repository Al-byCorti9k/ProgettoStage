package machinelearning

import (
	//"encoding/csv"
	"fmt"
	//"io"
	"math"

	//"strconv"
	"fortio.org/progressbar"
	//dataprocess "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/data_process"
	"github.com/go-gota/gota/dataframe"

	"github.com/sajari/regression"
)

// CSVToXY legge un file CSV, assume che la prima riga sia l'intestazione,
// e restituisce due slice:
//   - X: una slice di slice di float64 contenente i valori di tutte le colonne
//     eccetto quella specificata come target.
//   - Y: una slice di float64 contenente i valori della colonna target.
//
// La funzione converte tutti i valori in float64; se la conversione fallisce,
// restituisce un errore.
func DataFrameToXY(df *dataframe.DataFrame, targetColumn string) ([][]float64, []float64, error) {

	//Controlla che il DataFrame non sia vuoto
	if df.Nrow() == 0 {
		return nil, nil, fmt.Errorf("empty dataframe")
	}

	//Recupera i nomi delle colonne
	names := df.Names()

	//Cerca l'indice della colonna target
	targetIndex := -1
	for i, name := range names {
		if name == targetColumn {
			targetIndex = i
			break
		}
	}

	//Se non trova la colonna target restituisce errore
	if targetIndex == -1 {
		return nil, nil, fmt.Errorf("column '%s' not found", targetColumn)
	}

	//Numero di righe (samples)
	nRows := df.Nrow()

	//Numero totale di colonne
	nCols := df.Ncol()

	// Prealloca la matrice X:
	// - una riga per ogni sample
	// - una colonna in meno (escludiamo il target)
	X := make([][]float64, nRows)
	for i := range X {
		X[i] = make([]float64, nCols-1)
	}

	// Prealloca il vettore Y (target)
	Y := make([]float64, nRows)

	// Estrae l'intera colonna target come slice di float64
	// .Float() converte automaticamente int → float64
	targetCol := df.Col(targetColumn).Float()

	// Copia i valori nella slice Y
	copy(Y, targetCol)

	// Indice per riempire le colonne di X
	//(serve perché stiamo saltando la colonna target)
	xColIndex := 0

	// Itera su tutte le colonne del dataframe
	for j, name := range names {

		// Salta la colonna target
		if j == targetIndex {
			continue
		}

		// Estrae la colonna corrente come []float64
		col := df.Col(name).Float()

		// Copia i valori nella matrice X
		// riga per riga
		for i := 0; i < nRows; i++ {
			X[i][xColIndex] = col[i]
		}

		// Passa alla prossima colonna di X
		xColIndex++
	}

	// Restituisce:
	// X -> matrice features
	// Y -> vettore target
	return X, Y, nil
}

// funzione che prende i pesi, usando la riga lasciata fuori dal fold prova a predirre il valore corretto. fitIntercept è il parametro per l'inserimetno del bias
func Predict(x [][]float64, weights []float64, fitIntercept bool) float64 {
	//numero righe
	nSamples := len(x)
	//numero colonne. Ricorda che ha due dimensioni, ma è solo una riga
	nFeatures := len(x[0])
	//inizializiamo la matrice con le probabilità di ogni punto
	var probs float64

	for i := 0; i < nSamples; i++ {
		var dot float64
		//si moltiplica ogni valore della riga per i pesi.
		//i risultati si sommano cumulativamente
		//in sostanza stiamo calcolando il prodotto scalare tra le feature
		//e i relativi pesi. Il risultato ci porta alla probabilità
		//y = w_1*x_1 +...+ w_n*x_n + b (b è il bias)
		for j := 0; j < nFeatures; j++ {
			dot += x[i][j] * weights[j]
		}
		//si somma il fattore del bias se il parametro è settato
		if fitIntercept {
			dot += weights[nFeatures] // solo l’ultimo è intercept
		}

		probs = dot
	}
	//restituisce
	return probs
}

// MCC calcola il Matthews Correlation Coefficient
// yTrue: valori veri (0 o 1)
// yPred: valori predetti (0 o 1)
func MCC(yTrue []int, yPred []int) float64 {
	if len(yTrue) != len(yPred) {
		panic("yTrue e yPred lenghts should be equal")
	}

	var tp, tn, fp, fn float64
	//cicliamo per arricchire i valori relativi alle 4 categorie
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
	//calcolo numeratore e denominatore della formula di MCC
	num := tp*tn - fp*fn
	den := math.Sqrt((tp + fp) * (tp + fn) * (tn + fp) * (tn + fn))

	if den == 0 {
		return 0 //caso speciale, MCC non definito
	}
	//https://en.wikipedia.org/wiki/Phi_coefficient#:~:text=The%20MCC%20can%20be%20calculated%20directly%20from%20the%20confusion%20matrix%20using%20the%20formula
	return num / den
}

// LeaveOneOutCV esegue la Leave-One-Out cross-validation
// X: matrice delle feature
// y: etichette (0.0 o 1.0)
// ritorna slice di predizioni 0/1 e MCC
func LeaveOneOutCV(X [][]float64, y []float64) float64 {
	nSamples := len(X)
	predictions := make([]int, nSamples)

	// inizializza la load bar
	bar := progressbar.NewBar()
	// imposta il nome della load bar (viene mostrato a schermo)
	bar.UpdatePrefix("LOOCV progress: ")
	//defer è un'istruzione che etichetta le istruzioni che verranno eseguite solo alla fine del blocco in cui si trovano. Qui termina la load bar
	defer bar.End()
	// si cicla ogni linea dei sample
	for i := 0; i < nSamples; i++ {
		//serve per allineare la barra di caricamento con il fold in esame corrente
		percent := 100.0 * float64(i) / float64(nSamples)
		bar.Progress(percent)
		//indica il numero di fold attuale
		bar.UpdateSuffix(fmt.Sprintf("fold %d/%d", i+1, nSamples))

		X_train := make([][]float64, 0, nSamples-1)
		y_train := make([]float64, 0, nSamples-1)
		// per ogni fold, si esclude dal train set l'i-esimo campione del ciclo corrente. Alla fine ogni sample verrà usato per il test in uno specifico fold.
		for j := 0; j < nSamples; j++ {
			//arricchisce gli array sopra inizializzati con tutti i sample tranne quello attuale.
			if j != i {
				X_train = append(X_train, X[j])
				y_train = append(y_train, y[j])
			}
		}
		//per questa versione, userò l'implementazione della regressione lineare proveniente da:
		// github.com/sajari/regression

		//crea il modello di regressione
		r := new(regression.Regression)
		//imposta il nome della variabile osservata
		r.SetObserved("target")
		//imposta le variabili indipendenti
		for k := 0; k < len(X_train[0]); k++ {
			r.SetVar(k, fmt.Sprintf("var%d", k))
		}
		//aggiunge i punti di addestramento (quindi la riga e il valore target per quella riga)
		for idx, sample := range X_train {
			r.Train(regression.DataPoint(y_train[idx], sample))
		}
		//effettua la regressione
		r.Run()

		//predice il valore per il campione lasciato fuori
		predicted, _ := r.Predict(X[i])

		//sogliatura del 0.5, per ottenere le classi 1/0
		if predicted >= 0.5 {
			predictions[i] = 1
		} else {
			predictions[i] = 0
		}
	}

	bar.Progress(100)
	bar.Redraw()

	//conversione del target in int, per compatibilità per il calcolo dell'mcc
	yInt := make([]int, nSamples)
	for i, v := range y {
		if v >= 0.5 {
			yInt[i] = 1
		} else {
			yInt[i] = 0
		}
	}

	//calcola l'mcc
	mcc := MCC(yInt, predictions)
	return mcc
}
