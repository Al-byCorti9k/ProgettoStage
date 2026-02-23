package main

import (
	"fmt"
	"log"

	"os"

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
	fmt.Println(args.CArgs)
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
			targetColumn = args.CArgs[0]
		} else {
			_, numberCols := dfInfo.Df.Dims()
			targetColumn = dfInfo.Df.Names()[numberCols-1]
		}
		//qui avviene il preprocessing, quindi riempimento dei valori nulli, la scalatura standard e il one-hot-encoding
		dfInfoProcessed := dataprocess.FillColumnsNanValues(&dfInfo).StandardScalar().OneHotEncoding(targetColumn)
		//genero un file generico dove viene esportato il dataframe processato
		f, err := os.Create("output.csv")
		cli.HandleError(err)
		// 3. Esportiamo il DataFrame nel file
		err = dfInfoProcessed.Df.WriteCSV(f)
		cli.HandleError(err)

		X, Y, err := learning.CSVToXY("output.csv", targetColumn)
		//chiudo il file
		f.Close()
		cli.HandleError(err)

		//calcolo il valore di mcc
		mcc := learning.LeaveOneOutCV(X, Y)

		fmt.Println(mcc)
	}
}
