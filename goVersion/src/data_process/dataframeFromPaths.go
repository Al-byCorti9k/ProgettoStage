package dataprocess

import (
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"

	"github.com/go-gota/gota/dataframe"
)

// una funzione che restituisce il dataframe dal suo ID
func GetDataframeFromID(id int) dataframe.DataFrame {
	//ottengo il percorso dei csv
	targetPath := getCsvPath()
	//ottengo il nome del csv dall'ID
	csvName, err := GetDatasetInfo(&id)
	fmt.Println(csvName)
	if err != nil {
		var dsErr *DatasetError

		if errors.As(err, &dsErr) {
			fmt.Printf("%s\n", dsErr.Error())
		}
	}
	//fondo il percorso con il nome del csv scelto
	csvPath := filepath.Join(targetPath, csvName.GetCSV())

	//ora apro il file (panica se non ci riesce)
	reader, err := os.Open(csvPath)
	if err != nil {
		panic(err)
	}
	//defer, quando questa funzione smette di eseguire, lancia il comando per chiudere il file aperto
	defer reader.Close()
	//creo il dataframe dal reader
	df := dataframe.ReadCSV(reader, dataframe.WithDelimiter(','))
	//restituisco il dataframe creato
	return df
}

// ritorna il percorso della cartella contenente i csv
func getCsvPath() string {
	//ottengo la directory corrente
	currentDir, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}
	//risalgo alla directory dei dataset. Indipendentemente dall'OS
	targetPath := filepath.Join(currentDir, "..", "..", "data")

	//directory assoluta
	absPath, err := filepath.Abs(targetPath)
	if err != nil {
		log.Fatal(err)
	}

	return absPath
}
