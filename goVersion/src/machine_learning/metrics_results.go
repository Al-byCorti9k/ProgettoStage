package machinelearning

import (
	"encoding/csv"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"time"
)

// WriteCSV scrive i dati in formato CSV su un io.Writer (es. os.Stdout, file, buffer).
// è un'interfaccia di Go che va implementata per tipo, in questo caso l'ho implementata per ResultData
func (r *ResultData) WriteCSV(w io.Writer) error {
	//Crea un nuovo scrittore CSV che utilizzerà il writer fornito.
	//csv.NewWriter restituisce un oggetto che bufferizza internamente i dati
	//per efficienza.
	writer := csv.NewWriter(w)

	//defer writer.Flush() garantisce che, al termine della funzione,
	//tutti i dati ancora nel buffer interno vengano forzati a essere scritti
	//nel writer sottostante (ad esempio nel file). Se non chiamassimo Flush,
	//alcuni dati potrebbero rimanere in memoria e non essere effettivamente
	//salvati su disco.
	defer writer.Flush()

	//Scrive l'intestazione del CSV.
	//Write accetta una slice di stringhe e la trasforma in una riga CSV
	//(virgole come separatori, eventuali virgolette se necessario).
	if err := writer.Write([]string{
		"Dataset",
		"Operating system",
		"Column selected",
		"LOOCV's time execusion (s)",
		"LOOCV's time execution (ms)",
		"MCC",
		"energy consumption (kWh)",
		"methodology",
	}); err != nil {
		return err
	}

	//Itera su tutte le righe di dati presenti nella struct.
	//Le slice (Dataset, Os, TimeS, ecc.) hanno tutte la stessa lunghezza,
	//quindi possiamo usare l'indice i per accedere ai valori corrispondenti.
	for i := 0; i < len(r.Dataset); i++ {
		//Crea una slice di stringhe che rappresenta una riga del CSV.
		//I valori numerici (float64) devono essere convertiti in stringhe
		//perché il pacchetto csv lavora esclusivamente con stringhe.
		//fmt.Sprintf viene utilizzato proprio per questa conversione:
		//trasforma un float in una rappresentazione testuale.
		//Il formato "%f" produce una notazione decimale senza esponente,
		//con 6 cifre decimali di default
		record := []string{
			r.Dataset[i],                   //stringa, già pronta
			r.Os[i],                        //stringa
			r.TargetColumn[i],              //stringa
			fmt.Sprintf("%f", r.TimeS[i]),  //float64 → stringa
			fmt.Sprintf("%f", r.TimeMs[i]), //float64 → stringa
			fmt.Sprintf("%f", r.Mcc[i]),    //float64 → stringa
			fmt.Sprintf("%f", r.Energy[i]), //float64 → stringa
			fmt.Sprintf("%s", r.Method[i]), //anche se è stringa, lo convertiamo per sicurezza
		}

		//Scrive la riga nel buffer CSV.
		if err := writer.Write(record); err != nil {
			return err
		}
	}

	//Al termine del ciclo, il defer si occupa di chiamare Flush,
	//che svuota il buffer e scrive effettivamente tutti i dati.
	//Se non ci sono errori, restituisce nil.
	return nil
}

// PrintRows stampa i contenuti di ResultData in formato tabellare allineato.
// Viene calcolata la larghezza massima di ogni colonna considerando sia
// l'intestazione che i valori (per i numeri, si converte prima in stringa con
// la precisione voluta).
// Si usa %-*s per stampare stringhe allineate a sinistra con larghezza
// variabile. I numeri vengono convertiti in stringa con fmt.Sprintf() per
// garantire che la larghezza calcolata corrisponda esattamente a ciò che viene
// stampato.
func (r *ResultData) PrintRows() {
	if len(r.Dataset) == 0 {
		fmt.Println("There are no results")
		return
	}

	//Calcola la larghezza massima per ogni colonna (inclusa l'intestazione)
	maxLen := struct {
		dataset, os, target, timeS, timeMs, mcc, energy, method int
	}{
		dataset: len("Dataset"),
		os:      len("Operating system"),
		target:  len("Column selected"),
		timeS:   len("LOOCV's time execution (s)"),
		timeMs:  len("LOOCV's time execution (ms)"),
		mcc:     len("MCC"),
		energy:  len("energy consumption (kWh)"),
		method:  len("methodology"),
	}

	for i := 0; i < len(r.Dataset); i++ {
		//Stringhe
		if l := len(r.Dataset[i]); l > maxLen.dataset {
			maxLen.dataset = l
		}
		if l := len(r.Os[i]); l > maxLen.os {
			maxLen.os = l
		}
		if l := len(r.TargetColumn[i]); l > maxLen.target {
			maxLen.target = l
		}
		if l := len(r.Method[i]); l > maxLen.method {
			maxLen.method = l
		}
		//Numeri formattati
		if l := len(fmt.Sprintf("%.4f", r.TimeS[i])); l > maxLen.timeS {
			maxLen.timeS = l
		}
		if l := len(fmt.Sprintf("%.2f", r.TimeMs[i])); l > maxLen.timeMs {
			maxLen.timeMs = l
		}
		if l := len(fmt.Sprintf("%f", r.Mcc[i])); l > maxLen.mcc {
			maxLen.mcc = l
		}
		if l := len(fmt.Sprintf("%.4f", r.Energy[i])); l > maxLen.energy {
			maxLen.energy = l
		}
	}

	//Stampa intestazione (allineata a sinistra)
	fmt.Printf("%-*s %-*s %-*s %-*s %-*s %-*s %-*s %-*s\n",
		maxLen.dataset, "Dataset",
		maxLen.os, "Operating system",
		maxLen.target, "Column selected",
		maxLen.timeS, "LOOCV's time execution (s)",
		maxLen.timeMs, "LOOCV's time execution (ms)",
		maxLen.mcc, "MCC",
		maxLen.energy, "energy consumption (kWh)",
		maxLen.method, "methodology")

	//Stampa righe
	for i := 0; i < len(r.Dataset); i++ {
		fmt.Printf("%-*s %-*s %-*s %-*s %-*s %-*s %-*s %-*s\n",
			maxLen.dataset, r.Dataset[i],
			maxLen.os, r.Os[i],
			maxLen.target, r.TargetColumn[i],
			maxLen.timeS, fmt.Sprintf("%.4f", r.TimeS[i]),
			maxLen.timeMs, fmt.Sprintf("%.2f", r.TimeMs[i]),
			maxLen.mcc, fmt.Sprintf("%f", r.Mcc[i]),
			maxLen.energy, fmt.Sprintf("%.4f", r.Energy[i]),
			maxLen.method, r.Method[i])
	}
}

// ResultData corrisponde alla struct Rust con campi slice.
type ResultData struct {
	Dataset      []string
	Os           []string
	TimeS        []float64
	TimeMs       []float64
	TargetColumn []string
	Mcc          []float64
	Energy       []float64
	Method       []string
}

// AddEntry aggiunge una nuova riga di dati a tutte le slice.
// I parametri corrispondono ai valori di un singolo esperimento.
func (r *ResultData) AddEntry(dataset string, os string, timeS float64, timeMs float64, targetColumn string, mcc float64, energy float64, method string) {
	r.Dataset = append(r.Dataset, dataset)
	r.Os = append(r.Os, os)
	r.TargetColumn = append(r.TargetColumn, targetColumn)
	r.TimeS = append(r.TimeS, timeS)
	r.TimeMs = append(r.TimeMs, timeMs)
	r.Energy = append(r.Energy, energy)
	r.Mcc = append(r.Mcc, mcc)
	r.Method = append(r.Method, method)
}

// a seconda del sistema operativo, seleziona quale informazione riportare
// sulla tipologia di strumento che verrà usata per il calcolo dei consumi
// energetici
func SetMethod(os string) string {
	if os == "windows" {
		return "Intel's VTune Profiler"
	}
	return "RAPL interface"
}

// Salva la struct resultData in un csv nel persocorso results
func (results *ResultData) SaveResultsToPath() {
	//Ottieni la directory corrente
	currentDir, err := os.Getwd()
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}
	//Risali di due livelli (.. due volte)
	parentDir := filepath.Join(currentDir, "..", "results")

	//Genera il nome file con timestamp
	timestamp := time.Now().Format("2006-01-02_15h-04m-05s")
	filename := fmt.Sprintf("experiment_go_%s.csv", timestamp)

	//Combina percorso e nome file
	filePath := filepath.Join(parentDir, filename)

	//si assicura che la directory esista
	//7 (proprietario): lettura (4) + scrittura (2) + esecuzione (1) = 7
	//5 (gruppo): lettura (4) + esecuzione (1) = 5
	//5 (altri): lettura (4) + esecuzione (1) = 5
	if err := os.MkdirAll(parentDir, 0755); err != nil {
		fmt.Printf("Error creating directory: %v\n", err)
		return
	}

	//Crea il file
	file, err := os.Create(filePath)
	if err != nil {
		fmt.Printf("Error creating file: %v\n", err)
		return
	}
	//nella sintassi di GO, defer marchia le istruzioni che verranno eseguite solo al termine della funzione di dichiarazione
	defer file.Close()

	//Scrive il CSV
	if err := results.WriteCSV(file); err != nil {
		fmt.Printf("Error writing CSV: %v\n", err)
		return
	}

	fmt.Printf("File saved to: %s\n", filePath)
}
