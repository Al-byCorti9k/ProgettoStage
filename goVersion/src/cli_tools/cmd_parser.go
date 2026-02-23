// modulo per il parsing degli argomenti da linea di comando
// e per la verifica
package clitools

import (
	"errors"
	"fmt"
	"log"

	dataprocess "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/data_process"
	pflag "github.com/spf13/pflag"

	"github.com/go-gota/gota/dataframe"
)

// struttura dati che rappresenta i valori dati in input ai vari flags
type ConfigArgs struct {
	DArgs []int
	CArgs []string
	VArgs bool
}

// funzione che parsa gli argomenti da linea di comando ed effettua i controlli
func ParseCliArgument() ConfigArgs {

	var myArgs ConfigArgs
	//imposto il flag per ricevere i datasets
	pflag.IntSliceVarP(&myArgs.DArgs, "dataset", "d", nil, "select a dataset from its ID")

	//imposto il flag per ricevere il nome delle colonne
	pflag.StringArrayVarP(&myArgs.CArgs, "target_column", "t", nil, "Select the target column by its name. If multiple columns are selected and a column name contains a comma, use the following syntax:\n-t column1 -t column2\n Otherwise, you should use:\n-t column1,column2")

	pflag.BoolVarP(&myArgs.VArgs, "view_datasets", "v", false, "view all available datasets with their IDs")

	pflag.Parse()
	//effettuo il controllo degli argomenti. Il controllo sulla colonna è
	//effettuabile solo quando si crea il dataframe
	argsParse(&myArgs)

	//se non ci sono stati errori, ritorna gli argomenti parsati
	return myArgs
}

// funzione che controlla se gli argomenti siano validi
func argsParse(myArgs *ConfigArgs) {
	// Se isValid restituisce un errore, log.Fatal lo stampa e chiude il programma
	if err := myArgs.isValid(); err != nil {
		log.Fatalf("Configuration's error: %v", err)
	}

}

// funzione generica che con le mappe verifica se le slice di input abbiano duplicati
func hasDuplicates[T comparable](slice []T) bool {
	seen := make(map[T]bool)
	for _, v := range slice {
		if seen[v] {
			return true
		}
		seen[v] = true
	}
	return false
}

// Metodo per verificare duplicati in dArgs
func (c *ConfigArgs) hasDuplicateDatasets() bool {
	return hasDuplicates(c.DArgs)
}

// Metodo per verificare duplicati in cArgs
func (c *ConfigArgs) hasDuplicateTargets() bool {
	return hasDuplicates(c.CArgs)
}

// chiama le precedenti e verifica se tutte le condizioni sono rispettate
func (c *ConfigArgs) isValid() error {
	lenD := len(c.DArgs)
	lenT := len(c.CArgs)

	// 1. Controllo: d deve essere tra 0 e 4
	for _, id := range c.DArgs {
		if id < 0 || id > 4 {
			return fmt.Errorf("The dataset ID %d is not valid: it must be between 0 and 4.", id)
		}
	}

	// "Va bene se -t non c'è (0), ma se c'è deve avere lo stesso numero di -d"
	if lenT > 0 && lenT != lenD {
		return fmt.Errorf("Input mismatch: received %d datasets (-d) but %d target columns (-t).", lenD, lenT)
	}
	//non ci devono essere dataset duplicati.
	if c.hasDuplicateDatasets() {
		return errors.New("Error: Duplicated ID datasets.")
	}
	//non ci devono essere colonne target duplicate.
	if c.hasDuplicateTargets() {
		return errors.New("Error: Duplicated target column'names.")
	}
	return nil
}

// effettua verifiche sulle colonne di target selezionate
func InputColumnsCheck(dfInfo *dataprocess.DataframeInfo, columnName *string) {
	err := dfContainsTargetColumn(dfInfo, columnName)
	HandleError(err)

	err = targetColumnIsCategorical(dfInfo, columnName)

	HandleError(err)

	err = targetColumnIsBinary(dfInfo, columnName)

	HandleError(err)

}

// verifica che una colonna appartenga ad un dataframe
func dfContainsTargetColumn(dfInfo *dataprocess.DataframeInfo, columnName *string) error {

	r, c := dfInfo.Df.Select(*columnName).Dims()

	if r == 0 && c == 0 {
		return fmt.Errorf("target column \"%s\" doesn't belong to dataframe \"%s\"", *columnName, dfInfo.DfName)
	}

	return nil
}

// controlla se la colonna target è una colonna categorica
func targetColumnIsCategorical(dfInfo *dataprocess.DataframeInfo, columnName *string) error {

	//ottengo da dataframeInfo l'ID, con cui ottengo le colonne categoriche
	info, _ := dataprocess.GetDatasetInfo(&dfInfo.Id)

	_, ok := info.VecToHashSet()[*columnName]

	if !ok {
		return fmt.Errorf("target column \"%s\" is not a categorical column for \"%s\" dataset", *columnName, dfInfo.DfName)
	}

	return nil
}

// controlla se la colonna categorica target abbia solo due categorie (0 e 1)
func targetColumnIsBinary(dfInfo *dataprocess.DataframeInfo, columnName *string) error {
	//crea un gruppo sulla colonna su cui effettuare calcoli
	groups := dfInfo.Df.GroupBy(*columnName)
	//aggrega creando un nuovo dataframe con tante righe quante le differenti
	//categorie
	aggregatedDf := groups.Aggregation([]dataframe.AggregationType{dataframe.Aggregation_COUNT}, []string{*columnName})
	//calcoliamo il numero delle categorie attraverso il numero delle righe
	groupsNumber := aggregatedDf.Nrow()
	//se le righe non sono due, allora la colonna target non è bicategorica
	if groupsNumber != 2 {

		return fmt.Errorf("target column \"%s\" is not bicategorical", *columnName)
	}
	//estraiamo i valori della categoria come uno slice di stringhe
	categories := aggregatedDf.Col(*columnName).Records()

	// Verifichiamo se le categorie sono esattamente "0" e "1"
	// Usiamo una mappa per gestire il fatto che l'ordine potrebbe variare
	valid := true
	checkMap := map[string]bool{"0": false, "1": false}
	//se c'è anche un solo valore discordante, il ciclo di ferma
	for _, cat := range categories {
		if _, exists := checkMap[cat]; exists {
			checkMap[cat] = true
		} else {
			valid = false
			break
		}
	}

	// Controllo finale: devono essere validi e entrambi trovati (true)
	if !(valid && checkMap["0"] && checkMap["1"]) {
		return fmt.Errorf("Error: target column is not binary")
	}

	return nil
}

// per evitare di ripeterlo sempre
func HandleError(err error) {
	if err != nil {
		log.Fatalf("%+v", err)
	}
}

// serve all'utente per visualizzare tutti i dataset registrati
func DatasetView() {

	names := [5]string{
		"SEPSIS",
		"NEUROBLASTOMA",
		"DEPRESSION_HEART",
		"CARDIAC_ARREST",
		"DIABETES",
	}
	// Calcola la lunghezza massima dei nomi
	maxLen := 0
	for _, name := range names {
		if len(name) > maxLen {
			maxLen = len(name)
		}
	}

	// Stampa con allineamento: il nome occupa esattamente maxLen caratteri (allineato a sinistra)
	for i, name := range names {
		fmt.Printf("%-*s: %d\n", maxLen, name, i)
	}

}
