//modulo con la lista dei csv, le colonne categoriche

use std::collections::HashSet;

const DATASETS: [&str; 5] = [
    "journal.pone.0148699_S1_Text_Sepsis_SIRS_EDITED.csv",
    "10_7717_peerj_5665_dataYM2018_neuroblastoma.csv",
    "journal.pone.0158570_S2File_depression_heart_failure.csv",
    "journal.pone.0175818_S1Dataset_Spain_cardiac_arrest_EDITED.csv",
    "Takashi2019_diabetes_type1_dataset_preprocessed.csv",
];

//Definizione dello schema di un dataset. Ha due informazioni correlate:
// Nome del file.csv
// Array con il nome delle colonne di dati categorici. Serve per il One-hot encoding
pub struct DatasetInfo {
    file: &'static str,
    categorical_cols: &'static [&'static str],
}
//metodi associati al type DatasetInfo
impl DatasetInfo {
    //getter
    pub fn get_csv(&self) -> &'static str {
        &self.file
    }
    //getter
    pub fn get_cat_cols(&self) -> &'static [&'static str] {
        &self.categorical_cols
    }
}

// possibili errori di accesso ai dati
#[derive(Debug)]
pub enum DatasetError {
    //se un utente seleziona un dataset fuori dal range, ottiene questo errore
    IndexOutOfBounds { index: usize },
}

// preleva lo schema del dataset selezionato. UNICO PUNTO DI ACCESSO ESTERNO
pub fn get_dataset_info(index: Option<usize>) -> Result<&'static DatasetInfo, DatasetError> {
    //se l'input è nullo, l'indice si riferisce al primo dataset
    let idx = index.unwrap_or(0);
    DATASETS_INFO
        .get(idx)
        .ok_or(DatasetError::IndexOutOfBounds { index: idx })
}

// Istanza costante dei degli schemi dei csv supportati
const DATASETS_INFO: &[DatasetInfo] = &[
    DatasetInfo {
        file: DATASETS[0],
        categorical_cols: &["sex_woman", "diagnosis_0EC_1M_2_AC", "Group", "Mortality"],
    },
    DatasetInfo {
        file: DATASETS[1],
        categorical_cols: &[
            "age",
            "sex",
            "site",
            "stage",
            "risk",
            "autologous_stem_cell_transplantation",
            "radiation",
            "degree_of_differentiation",
            "UH_or_FH",
            "MYCN_status ",
            "surgical_methods",
            "outcome",
        ],
    },
    DatasetInfo {
        file: DATASETS[2],
        categorical_cols: &[
            "Male (1=Yes, 0=No)",
            "Etiology HF(1=Yes, 0=No)",
            "Death (1=Yes, 0=No)",
            "Hospitalized (1=Yes, 0=No)",
        ],
    },
    DatasetInfo {
        file: DATASETS[3],
        categorical_cols: &[
            "Exitus",
            "sex_woman",
            "Endotracheal_intubation",
            "Functional_status",
            "Asystole",
            "Bystander",
            "Cardiogenic",
            "Cardiac_arrest_at_home",
        ],
    },
    DatasetInfo {
        file: DATASETS[4],
        categorical_cols: &["sex_0man_1woman", "insulin_regimen_binary"],
    },
];

//i trait sono simili come concetto alle interfacce di Java. In questo caso
//espandiamo le funzionalità con un metodo per convertire le slice di stringhe
//in HashSet. Servirà per avere accesso alle colonne categoriche in tempo
// O(1)
pub trait VecToHash {
    fn vec_to_hashset(&self) -> HashSet<&'static str>;
}

pub trait VecToHashSet {
    fn vec_to_hashset(&self) -> HashSet<&str>;
}

impl VecToHash for &'static [&'static str] {
    fn vec_to_hashset(&self) -> HashSet<&'static str> {
        self.iter().copied().collect()
    }
}

impl VecToHashSet for Vec<&str> {
    fn vec_to_hashset(&self) -> HashSet<&str> {
        self.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_process::errors::AppError;

    use super::*;

    #[test]
    fn test_conversione_to_hashset() -> Result<(), AppError> {
        let pippo = get_dataset_info(Some(0))?.get_cat_cols();
        let franco = pippo.vec_to_hashset();

        let prova = franco.get("sex_woman").copied().unwrap();
        assert_eq!("sex_woman", prova);
        let owner = get_dataset_info(Some(0))?.get_cat_cols()[0];
        assert_eq!("sex_woman", owner);
        Ok(())
    }
}
