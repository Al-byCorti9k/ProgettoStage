package dataprocess

import (
	"fmt"
)

// Nomi dei file csv disponibili
var dataset = [5]string{
	"journal.pone.0148699_S1_Text_Sepsis_SIRS_EDITED.csv",
	"10_7717_peerj_5665_dataYM2018_neuroblastoma.csv",
	"journal.pone.0158570_S2File_depression_heart_failure.csv",
	"journal.pone.0175818_S1Dataset_Spain_cardiac_arrest_EDITED.csv",
	"Takashi2019_diabetes_type1_dataset_preprocessed.csv",
}

// DatasetInfo definisce lo schema di un dataset.
type DatasetInfo struct {
	file            string
	categoricalCols []string
}

// GetCSV restituisce il nome del file (Getter)
func (d *DatasetInfo) GetCSV() string {
	return d.file
}

// GetCatCols restituisce le colonne categoriche (Getter)
func (d *DatasetInfo) GetCatCols() []string {
	return d.categoricalCols
}

// DatasetError gestisce gli errori di accesso (implementa l'interfaccia error di Go)
type DatasetError struct {
	Index int
}

func (e *DatasetError) Error() string {
	return fmt.Sprintf("Index out of bounds: %d", e.Index)
}

// DATASETS_INFO: Slice contenente le configurazioni dei dataset
var DATASETS_INFO = []DatasetInfo{
	{
		file:            dataset[0],
		categoricalCols: []string{"sex_woman", "diagnosis_0EC_1M_2_AC", "Group", "Mortality"},
	},
	{
		file:            dataset[1],
		categoricalCols: []string{"age", "sex", "site", "stage", "risk", "autologous_stem_cell_transplantation", "radiation", "degree_of_differentiation", "UH_or_FH", "MYCN_status ", "surgical_methods", "outcome"},
	},
	{
		file:            dataset[2],
		categoricalCols: []string{"Male (1=Yes, 0=No)", "Etiology HF(1=Yes, 0=No)", "Death (1=Yes, 0=No)", "Hospitalized (1=Yes, 0=No)"},
	},
	{
		file:            dataset[3],
		categoricalCols: []string{"Exitus", "sex_woman", "Endotracheal_intubation", "Functional_status", "Asystole", "Bystander", "Cardiogenic", "Cardiac_arrest_at_home"},
	},
	{
		file:            dataset[4],
		categoricalCols: []string{"sex_0man_1woman", "insulin_regimen_binary"},
	},
}

// GetDatasetInfo preleva lo schema del dataset selezionato.
func GetDatasetInfo(index *int) (*DatasetInfo, error) {
	idx := 3 // Valore di default
	if index != nil {
		idx = *index
	}

	if idx < 0 || idx >= len(DATASETS_INFO) {
		return nil, &DatasetError{Index: idx}
	}

	return &DATASETS_INFO[idx], nil
}

// struct{} non occupa memoria. Ideale per gli hashset
type HashSet[T comparable] map[T]struct{}

// converte un'array in un hashset, ossia una mappa con sole chiavi
// T è un generico: accetta tutti i tipi che implementano l'interfaccia comparable
func (d *DatasetInfo) VecToHashSet() HashSet[string] {
	slice := d.GetCatCols()
	set := make(HashSet[string], len(slice))
	for _, v := range slice {
		set[v] = struct{}{}
	}
	return set
}
