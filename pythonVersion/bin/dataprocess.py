#le funzioni implementano un preprocessing per ciascuno dei dataset
# serve per distinguere i dati categorici da quelli che non lo sono

from sklearn.impute import SimpleImputer
from sklearn.pipeline import make_pipeline, Pipeline
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef


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
        
        if columnName in dataFrame.columns.tolist():
            if dataFrame[columnName].dtype == "category":
                x_predictor = dataFrame.drop(columnName, axis=1)
                y_response = dataFrame[columnName]
            else:
                exit("\nla colonna selezionata \""+columnName+"\" non è categorica\n")
        else:
            exit("\nla colonna selezionata \""+columnName+"\" non è presente nel dataset\n")

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

from codecarbon import OfflineEmissionsTracker
tracker = OfflineEmissionsTracker(country_iso_code="ITA")

 # uso la classificazione con regressione logistica e LOOCV 
model = linear_model.LogisticRegression(max_iter = 1000)
cvp = model_selection.LeaveOneOut()

# preprocessare i dati è fondamentale per rendere comparabili i valori 
# categorici con quelli numerici, i quali a loro volta vengono scalati 
# per permettere un confronto adeguato

# creo una pipeline che effettua il preprocessing e poi applica il modello
clf = make_pipeline(preprocessor, model)

# viene effettuata la LOOCV predittiva, in modo da ottenere 
# le previsioni di ciascun fold, per poi valutare la prestazione
# del modello con la metrica MCC


def Logistic_Regression_Validation(x_predictor, y_response):
   

  #TODO capire come sistemare la stampa dei messaggi, non mi piace per niente
  # questa gestione poco trasparente e che su windows non può effettuare misure precise
  # perchè si affida completamente ad una componente deprecata 
    tracker.start()
    y_predict = model_selection.cross_val_predict(clf, x_predictor, y_response, cv = cvp )
    tracker.stop()
    
    time = model_selection.cross_validate(clf, x_predictor, y_response, cv = cvp)
    
 
    
    return y_predict, time