package main

import (
	"bytes"
	"fmt"
	"log"
	"runtime"
	"time"

	dataprocess "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/data_process"

	cli "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/cli_tools"

	learning "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/machine_learning"
)

func main() {
	//parso gli input da linea di comando
	args := cli.ParseCliArgument()
	//settaggi dei log. Personalizzazione stampa errori
	log.SetPrefix("go_version: ")
	log.SetFlags(0)
	if args.VArgs {
		cli.DatasetView()
		return
	}
	var results learning.ResultData
	//ciclo su tutti i dataset inseriti in input dall'utente. i è un'indice di
	//ciclo generato automaticamente da range, viene usato per riferirsi all'enentuale colonna target indicata in input.
	for i, dataset := range args.DArgs {

		//ottengo il dataframe
		df := dataprocess.GetDataframeFromID(dataset)
		//stampiamo il dataframe selezionato
		fmt.Println(df)
		//ottengo le informazioni del dataframe
		dfInfo := dataprocess.DataframeInfoBuild(dataset, &df)
		//dichiaro la variabile con il nome della colonna target
		var targetColumn string
		//se in input non ho dichiarato il nome, di default viene scelta come target
		//l'ultima colonna. Altrimenti viene controllato se gli input
		//dell'utente sono coerenti
		if len(args.CArgs) != 0 {
			cli.InputColumnsCheck(&dfInfo, &args.CArgs[i])
			targetColumn = args.CArgs[i]
		} else {
			_, numberCols := dfInfo.Df.Dims()
			targetColumn = dfInfo.Df.Names()[numberCols-1]
		}
		//qui avviene il preprocessing, quindi riempimento dei valori nulli, la scalatura standard e il one-hot-encoding
		dfInfoProcessed := dataprocess.FillColumnsNanValues(&dfInfo).StandardScalar().OneHotEncoding(targetColumn)
		//genero un file generico dove viene esportato il dataframe processato
		var buf bytes.Buffer
		//Esportiamo il DataFrame nel file
		err := dfInfoProcessed.Df.WriteCSV(&buf)
		cli.HandleError(err)

		X, Y, err := learning.CSVToXY(&buf, targetColumn)
		cli.HandleError(err)

		//calcolo il valore di mcc e il tempo di esecuzione
		inizio := time.Now()
		mcc := learning.LeaveOneOutCV(X, Y)
		fine := time.Since(inizio)
		//ad ogni ciclo, arricchisco le entry per il risultato.
		//serviranno poi per produrre il csv finale con i risultati
		results.AddEntry(
			dfInfoProcessed.DfName,
			runtime.GOOS,
			fine.Seconds(),
			float64(fine.Milliseconds()),
			targetColumn,
			mcc,
			0.0,
			learning.SetMethod(runtime.GOOS),
		)
	}
	//stampa i risultati a schermo
	results.PrintRows()
	// Salva su file
	results.SaveResultsToPath()
}
