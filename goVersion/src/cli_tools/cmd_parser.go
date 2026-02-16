// modulo per il parsing degli argomenti da linea di comando
// e per la verifica
package clitools

import (
	"errors"
	"fmt"
	"log"

	pflag "github.com/spf13/pflag"
)

// struttura dati che rappresenta i valori dati in input ai vari flags
type ConfigArgs struct {
	DArgs []int
	CArgs []string
}

func ParseCliArgument() ConfigArgs {

	var myArgs ConfigArgs
	//imposto il flag per ricevere i datasets
	pflag.IntSliceVarP(&myArgs.DArgs, "dataset", "d", nil, "select a dataset from its number")

	//imposto il flag per ricevere il nome delle colonne
	pflag.StringSliceVarP(&myArgs.CArgs, "target_column", "t", nil, "select target column from its name")

	pflag.Parse()
	//effettuo il controllo degli argomenti. Il controllo sulla colonna è
	//effettuabile solo quando si crea il dataframe
	ArgsParse(&myArgs)

	//se non ci sono stati errori, ritorna gli argomenti parsati
	return myArgs
}

func ArgsParse(myArgs *ConfigArgs) {
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
