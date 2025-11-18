#le funzioni implementano un preprocessing per ciascuno dei dataset
# serve per distinguere i dati categorici da quelli che non lo sono

datasets = ["journal.pone.0148699_S1_Text_Sepsis_SIRS_EDITED.csv",
            "10_7717_peerj_5665_dataYM2018_neuroblastoma.csv",
            "journal.pone.0158570_S2File_depression_heart_failure.csv",
            "journal.pone.0175818_S1Dataset_Spain_cardiac_arrest_EDITED.csv",
            "Takashi2019_diabetes_type1_dataset_preprocessed.csv"
            ]

categoryData = {
    datasets[0]: 
            ['sex_woman', 'diagnosis_0EC_1M_2_AC','Group', 'Mortality'],
    datasets[1]: 
            ['age', 'sex', 'site', 'stage', 'risk',
                'autologous_stem_cell_transplantation', 'radiation',
                'degree_of_differentiation', 'UH_or_FH', 'MYCN_status ',
                'surgical_methods', 'outcome'],
    datasets[2]:
            ['Male', 'Etiology', 'Death', 'Hospitalized'],
    
    datasets[3]:
            [ 'Exitus', 'sex_woman', 'Endotracheal_intubation',
               'Functional_status', 'Asystole', 'Bystander',
               'Cardiogenic', 'Cardiac_arrest_at_home' ],
    
    datasets[4]:
            ['sex_0man_1woman', 'insulin_regimen_binary']
        }
#funzione che seleziona il dataset scelto dall'utente e distingue colonne
#categoriche da quelle numeriche.
def datasetsSelection(selectedDataset):
    selectedDataset += 1
    cat_cols = categoryData[datasets[selectedDataset]]
    dtype_dict = {col: "category" for col in cat_cols}
    name_csv = datasets[selectedDataset]
    return dtype_dict, name_csv

#funzione che ritorna i dataframe divisi in gruppo da predirre e 
#gruppo da usare per la predizione.
def columnPredictionSelect(columnName, dataFrame ):
    x_predictor = dataFrame.drop(columnName, axis=1)
    y_response = dataFrame[columnName]

    return x_predictor, y_response






