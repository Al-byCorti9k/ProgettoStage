#le funzioni implementano un preprocessing per ciascuno dei dataset
# serve per distinguere i dati categorici da quelli che non lo sono

from sklearn.impute import SimpleImputer
from sklearn.pipeline import make_pipeline, Pipeline
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector



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
    selectedDataset -= 1
    cat_cols = categoryData[datasets[selectedDataset]]
    dtype_dict = {col: "category" for col in cat_cols}
    name_csv = datasets[selectedDataset]
    return dtype_dict, name_csv


#funzione che ritorna i dataframe divisi in gruppo da predirre e 
#gruppo da usare per la predizione.
def columnPredictionSelect(columnName, dataFrame ):
    #controllo se esiste la colonna selezionata ed è di tipo category
    if columnName != None:
        
        condition1 = dataFrame[columnName].dtype == "category"
        condition2 = columnName in dataFrame
        
        if condition1 and condition2:
            x_predictor = dataFrame.drop(columnName, axis=1)
            y_response = dataFrame[columnName]
        else:
            if condition1 == False:
                exit("la colonna selezionata \""+columnName+"\" non è categorica")
            else:
                exit("la colonna selezionata \""+columnName+"\" non è presente nel dataset")

    else:
        #caso di default, dove scelgo l'ultima colonna
        x_predictor = dataFrame[dataFrame.columns[:-1]]
        y_response =  dataFrame[dataFrame.columns[-1]]
        
    return x_predictor, y_response



#funzionalità per il preprocessing del dataframe
numeric_trasformer = Pipeline(steps =
                              [ 
                            ('mean', SimpleImputer(strategy="mean")),
                            ('scaler', StandardScaler())
                              ])

categorical_trasformer = Pipeline(steps = 
                             [
                                 ('moda', SimpleImputer(strategy="most_frequent")),
                                 ('onehot', OneHotEncoder(handle_unknown="ignore"))
                             ]     
                                  
                                  )
preprocessor = ColumnTransformer(
    transformers=[
        ('num', numeric_trasformer, selector(dtype_exclude="category")),
        ('cat', categorical_trasformer, selector(dtype_include="category"))
    ]
)
