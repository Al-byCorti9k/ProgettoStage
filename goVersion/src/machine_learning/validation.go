package machinelearning

import (
	"encoding/csv"
	"fmt"
	"io"
	"math"

	"strconv"

	"fortio.org/progressbar"
	"gonum.org/v1/gonum/optimize"
)

// CSVToXY legge un file CSV, assume che la prima riga sia l'intestazione,
// e restituisce due slice:
//   - X: una slice di slice di float64 contenente i valori di tutte le colonne
//     eccetto quella specificata come target.
//   - Y: una slice di float64 contenente i valori della colonna target.
//
// La funzione converte tutti i valori in float64; se la conversione fallisce,
// restituisce un errore.
func CSVToXY(r io.Reader, targetColumn string) ([][]float64, []float64, error) {
	// Apre il file. È importante gestire l'errore e chiudere il file con defer.
	//file, err := os.Open(filename)
	reader := csv.NewReader(r)
	records, err := reader.ReadAll()
	if err != nil {
		return nil, nil, err
	}
	//defer file.Close()
	// Crea un lettore CSV e legge tutto il contenuto in memoria.
	//reader := csv.NewReader(file)
	//records, err := reader.ReadAll()
	//if err != nil {
	//	return nil, nil, err
	//}
	// Controlla che ci sia almeno un'intestazione più una riga di dati.
	if len(records) < 2 {
		return nil, nil, fmt.Errorf("il CSV non contiene dati")
	}
	//La prima riga è l'intestazione.
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
	// Prealloca X e Y con la capacità giusta per evitare riallocazioni.
	X := make([][]float64, len(data))
	Y := make([]float64, len(data))
	// Itera su ogni riga di dati.
	for i, row := range data {
		// Verifica che la riga abbia lo stesso numero di colonne dell'intestazione.
		if len(row) != len(header) {
			return nil, nil, fmt.Errorf("numero colonne inconsistente alla riga %d", i+1)
		}
		// Converte il valore della colonna target in float64.
		Yval, err := strconv.ParseFloat(row[targetIndex], 64)
		if err != nil {
			return nil, nil, fmt.Errorf("errore conversione target riga %d: %v", i+1, err)
		}
		Y[i] = Yval
		//Costruisce la riga per X: tutte le colonne tranne quella target.
		newRow := make([]float64, 0, len(row)-1)
		for j, value := range row {
			if j == targetIndex {
				continue
			}
			// Converte il valore in float64.
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

// implementazione della funzione sigmoid
// https://en.wikipedia.org/wiki/Sigmoid_function
func sigmoid(z float64) float64 {

	if z >= 0 {
		ez := math.Exp(-z)
		return 1 / (1 + ez)
	}
	ez := math.Exp(z)
	return ez / (1 + ez)

}

// Sono ispirati a Scikit-learn:
// https://scikit-learn.org/stable/modules/linear_model.html#logistic-regression:~:text=1%2E1%2E11%2E1%2E%20Binary%20Case
type LogisticParams struct {
	Penalty      string  //coefficiente di penalità. guarda l'eq al link sopra. "l2" è il default
	C            float64 //termine di regolarizzazione
	FitIntercept bool    //specifica se il bias debba essere aggiunto alla funzione di decisione
	Tol          float64 //tolleranza per il criterio di stop
	MaxIter      int     //numero massimo di iterazioni
	//ClassWeight  map[float64]float64 rimosso perchè non sicuro della sua utilità
}

// funzione per il calcolo della regressione logistica, ispirata alla versione
// scikit learn. Le spiegazioni dei parametri le si trovano nella definizione
// della struct LogisticParams; consultare il link per dettagli utili
// https://scikit-learn.org/stable/modules/linear_model.html#logistic-regression:~:text=1%2E1%2E11%2E1%2E%20Binary%20Case
func LogisticRegression(x [][]float64, y []float64, params LogisticParams,
) []float64 {

	nSamples := len(x)
	nFeatures := len(x[0])
	//termine di regolarizzazione. se è zero viene settato almeno ad 1
	if params.C == 0 {
		params.C = 1.0
	}

	// sklearn: lambda = 1 / (C * nSamples)
	//dalla formula che viene usata da scikit learn per il bias
	lambda := 1.0 / (params.C * float64(nSamples))

	dim := nFeatures
	//viene aumentato il numero di feature artificialmente per aggiungere
	//spazio al bias. Se è settato
	if params.FitIntercept {
		dim++
	}
	//il problema è composto da due elementi:
	//la funzione che deve essere ottimizzata
	//il gradiente, che serve per calcolare la direzione di crescita dei
	//pesi, con l'obiettivo di minimizzarli riducendo l'errore.
	//l'obiettivo è trovare i pesi che diminuiscono l'errore della
	//funzione di loss (log-loss cross entropy)
	problem := optimize.Problem{

		Func: func(w []float64) float64 {
			//inizializziamo il parametro relativo al valore assunto dalla funzione loss. quello che vogliamo minimizzare
			var loss float64
			//calcoliamo le probabilità. Usiamo la stessa formula
			//usata in Predict, quella della regressione lineare
			for i := 0; i < nSamples; i++ {
				//viene fatta la combinazione lineare
				var dot float64
				for j := 0; j < nFeatures; j++ {

					dot += x[i][j] * w[j]
				}
				if params.FitIntercept {
					dot += w[nFeatures]
				}
				//otteniamo la probabilità
				prob := sigmoid(dot)
				weight := 1.0
				/*
					if params.ClassWeight != nil {
						weight = params.ClassWeight[y[i]]
					}
				*/
				//Misura quanto le probabilità predette p si discostano dalle vere etichette
				loss -= weight * (y[i]*math.Log(prob+1e-15) +
					(1-y[i])*math.Log(1-prob+1e-15))
			}
			//dividiamo la loss per il numero di sample
			loss /= float64(nSamples)
			//se la penalità è settata, viene applicata la formula, consultabile
			//anche dal link che ho inserito nella definizione della struct (LogisticParams)
			if params.Penalty == "l2" {
				for j := 0; j < nFeatures; j++ {
					//la formula
					loss += 0.5 * lambda * w[j] * w[j]
				}
			}
			//ritorna il valore di loss
			return loss
		},
		// calcola la derivata della loss rispetto a ogni peso.
		// l'approccio è analitico: viene usata la formula esatta
		// e serve al solver lbfgs per individuare le direzioni di discesa
		// in modo efficiente
		Grad: func(grad, w []float64) {
			// Il vettore `grad` deve contenere le derivate parziali della funzione di loss
			// rispetto a ciascun peso (e, opzionalmente, al bias). All'ingresso della funzione
			// `grad` è un'area di memoria già allocata ma con valori indeterminati. Dobbiamo
			// inizializzarla a zero prima di accumulare i contributi.
			for j := range grad {
				grad[j] = 0
			}
			// Itera su tutti i campioni del training set (nSamples) per calcolare il contributo
			// di ogni campione al gradiente.
			for i := 0; i < nSamples; i++ {
				var dot float64
				for j := 0; j < nFeatures; j++ {
					dot += x[i][j] * w[j]
				}
				if params.FitIntercept {
					dot += w[nFeatures] //memorizza il bias
				}
				//ottiene la probabilità predetta schiacciandola
				//con valori compresi da 0 e 1
				prob := sigmoid(dot)
				// Questo valore è la base per il gradiente della log‑loss.
				diff := prob - y[i]

				weight := 1.0
				/*
					if params.ClassWeight != nil {
						weight = params.ClassWeight[y[i]]
					}
				*/
				// Aggiorna il gradiente per ogni peso relativo alle feature (j = 0..nFeatures-1).
				// La formula analitica della derivata della log‑loss (senza regolarizzazione) per il peso w_j è:  ∂L/∂w_j = (1/n) * Σ (h_i - y_i) * x_{i,j}
				// Qui stiamo accumulando la somma non ancora divisa per n.
				for j := 0; j < nFeatures; j++ {
					grad[j] += weight * diff * x[i][j]
				}
				// Se è incluso il bias, aggiorniamo anche la sua derivata.
				// La derivata rispetto al bias è: (1/n) * Σ (h_i - y_i)
				if params.FitIntercept {
					grad[nFeatures] += weight * diff
				}
			}
			// Dopo aver sommato il contributo di tutti i campioni, dividiamo
			// ciascuna componente del gradiente per il numero di campioni
			//  nSamples.
			// In questo modo otteniamo il gradiente medio della loss (senza regolarizzazione).
			for j := 0; j < len(grad); j++ {
				grad[j] /= float64(nSamples)
			}
			// Se è attiva la regolarizzazione L2, aggiungiamo il gradiente del termine di penalty.
			// La loss regolarizzata è: L_tot = L_orig + (λ/2) * Σ w_j²  (solo sui pesi, non sul bias).
			// La derivata aggiuntiva per w_j è: λ * w_j.
			// Nota: λ = 1 / (C * nSamples) come calcolato all'inizio di LogisticRegression.
			if params.Penalty == "l2" {
				for j := 0; j < nFeatures; j++ {
					grad[j] += lambda * w[j]
				}
				// Il bias NON viene regolarizzato (come in scikit‑learn), quindi nessuna aggiunta per grad[nFeatures].
			}
		},
	}

	// Imposta MaxIter = 50 per essere identici a scikit-learn
	settings := optimize.Settings{
		MajorIterations:   params.MaxIter,
		GradientThreshold: params.Tol,
	}

	result, _ := optimize.Minimize(
		problem,
		make([]float64, dim),
		&settings,
		&optimize.LBFGS{},
	)

	return result.X
}

// funzione che prende i pesi, usando la riga lasciata fuori dal fold prova a predirre il valore corretto. fitIntercept è il parametro per l'inserimetno del bias
func Predict(x [][]float64, weights []float64, fitIntercept bool) float64 {
	//numero righe
	nSamples := len(x)
	//numero colonne. RIcorda che ha due dimensioni, ma è solo una riga
	nFeatures := len(x[0])
	//inizializiamo la matrice con le probabilità di ogni punto
	var probs float64

	for i := 0; i < nSamples; i++ {
		var dot float64
		//si moltiplica ogni valore della riga per i pesi.
		//i risultati si sommano cumulativamente
		//in sostanza stiamo calcolando il prodotto scalare tra le feature
		// e i relativi pesi. Il risultato ci porta alla probabilità
		// y = w_1*x_1 +...+ w_n*x_n + b (b è il bias)
		for j := 0; j < nFeatures; j++ {
			dot += x[i][j] * weights[j]
		}
		//si somma il fattore del bias se il parametro è settato
		if fitIntercept {
			dot += weights[nFeatures] // solo l’ultimo è intercept
		}
		//Otteniamo la probabilità dalla funzione sigmoid. schiaccia
		//le probabilità in valori continui compresi tra 0 e 1
		probs = sigmoid(dot)
	}
	//restituisce
	return probs
}

/*
func PredictClass(x [][]float64, weights []float64, fitIntercept bool) []int {

	probs := Predict(x, weights, fitIntercept)
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
*/
//MCC calcola il Matthews Correlation Coefficient
//yTrue: valori veri (0 o 1)
//yPred: valori predetti (0 o 1)
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
		return 0 // caso speciale, MCC non definito
	}
	//https://en.wikipedia.org/wiki/Phi_coefficient#:~:text=The%20MCC%20can%20be%20calculated%20directly%20from%20the%20confusion%20matrix%20using%20the%20formula
	return num / den
}

// LeaveOneOutCV esegue la Leave-One-Out cross-validation
// X: matrice delle feature
// y: etichette (0.0 o 1.0)
// ritorna slice di predizioni 0/1 e MCC
func LeaveOneOutCV(X [][]float64, y []float64) float64 {
	//settiamo il numero di fold da creare
	nSamples := len(X)
	//inizializziamo il vettore dei valori predetti da ciascun fold
	predictions := make([]int, nSamples)

	//Crea una nuova barra di progresso con le impostazioni predefinite
	bar := progressbar.NewBar()
	//Opzionale: personalizza il prefisso per indicare cosa stiamo facendo
	bar.UpdatePrefix("LOOCV progress: ")
	//Alla fine della funzione, assicurati di terminare la barra
	defer bar.End()

	//cicliamo sul numero di fold
	for i := 0; i < nSamples; i++ {

		//Calcola la percentuale completata (da 0 a 100)
		percent := 100.0 * float64(i) / float64(nSamples)
		//Aggiorna la barra di progresso
		bar.Progress(percent)

		//mostra il numero del fold corrente nel suffisso
		bar.UpdateSuffix(fmt.Sprintf("fold %d/%d", i+1, nSamples))
		//Creiamo X_train e y_train escludendo l'i-esimo esempio
		X_train := make([][]float64, 0, nSamples-1)
		y_train := make([]float64, 0, nSamples-1)
		//questo secondo ciclo serve per arricchire i due set sopra
		for j := 0; j < nSamples; j++ {
			//viene esclusa solo al riga i-esima, come da definizione di LOOCV
			//escludiamo solo una riga per fold.
			if j != i {
				X_train = append(X_train, X[j])
				y_train = append(y_train, y[j])
			}
		}
		//inizializziamo i parametri per la chiamata alla funzione custom della
		//regressione.
		params := LogisticParams{
			Penalty:      "l2",
			C:            1,
			FitIntercept: true,
			Tol:          0.0001,
			MaxIter:      50,
		}

		//Addestriamo il modello sui n-1 esempi con i parametri
		weights := LogisticRegression(X_train, y_train, params)

		//Prediciamo solo per l'esempio lasciato fuori
		prob := Predict([][]float64{X[i]}, weights, params.FitIntercept)

		//Convertiamo in 0 o 1 usando soglia 0.5
		if prob >= 0.5 {
			predictions[i] = 1
		} else {
			predictions[i] = 0
		}
	}
	bar.Progress(100)
	bar.Redraw()
	//Convertiamo y in []int
	yInt := make([]int, nSamples)
	for i, v := range y {
		if v >= 0.5 {
			yInt[i] = 1
		} else {
			yInt[i] = 0
		}
	}

	//Calcoliamo MCC
	mcc := MCC(yInt, predictions)

	return mcc
}
