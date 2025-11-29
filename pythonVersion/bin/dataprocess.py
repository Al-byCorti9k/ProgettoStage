#le funzioni implementano un preprocessing per ciascuno dei dataset
# serve per distinguere i dati categorici da quelli che non lo sono
from sklearn.impute import SimpleImputer
from sklearn.pipeline import make_pipeline, Pipeline
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef
import platform
import esternProcess
import time

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
            ['Male (1=Yes, 0=No)', 'Etiology HF(1=Yes, 0=No)', 'Death (1=Yes, 0=No)', 'Hospitalized (1=Yes, 0=No)'],
    
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



# Funzione per la gestione della scelta di colonne multiple per dataset multipli. il comando è del tipo
# python Main.py -i 1 2 3 4 -cn "pippo" "pluto" "paperino". Nota che la quarta fa il default
class staticLen:
    len = 0   
def columnPredictionIndexExtractor(columnName):
    if columnName != None:
        if staticLen.len >= len(columnName):
            columnIndex = -1
        else:
            columnIndex = staticLen.len
            staticLen.len += 1
    else:
        columnIndex = -1
    
    return columnIndex
     

#funzione che ritorna i dataframe divisi in gruppo da predirre e 
#gruppo da usare per la predizione.
def columnPredictionSelect(columnName, dataFrame ):
    columnIndex = columnPredictionIndexExtractor(columnName)
    columnNotExist = False
    columnNonCat = False
    x_predictor = None
    y_response = None
    #controllo se esiste la colonna selezionata ed è di tipo category
    if columnName != None and columnIndex != -1:
        
        if columnName[columnIndex] in dataFrame.columns.tolist():
            if dataFrame[columnName[columnIndex]].dtype == "category":
                x_predictor = dataFrame.drop(columnName[columnIndex], axis=1)
                y_response = dataFrame[columnName[columnIndex]]
            else:
                print("\nla colonna selezionata \""+columnName[columnIndex]+"\" non è categorica\n")
                columnNotExist = True
        else:
            print("\nla colonna selezionata \""+columnName[columnIndex]+"\" non è presente nel dataset\n")
            columnNonCat =  True
    else:
        #caso di default, dove scelgo l'ultima colonna
        x_predictor = dataFrame[dataFrame.columns[:-1]]
        y_response =  dataFrame[dataFrame.columns[-1]]
        if y_response.dtype != "category":
            columnNonCat = True
            print("\nla colonna selezionata \""+dataFrame.columns[-1]+"\" non è sus categorica\n")
    return x_predictor, y_response, columnNotExist, columnNonCat



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
    start = time.time()
    y_predict = model_selection.cross_val_predict(clf, x_predictor, y_response, cv = cvp )
    end = time.time()
    
    #time = model_selection.cross_validate(clf, x_predictor, y_response, cv = cvp)
    seconds = end - start
 
    
    return y_predict, seconds

def energyConsumption(operatingSystem, activation, forceCodeCarbon, name_csv, x_predictor, y_response ):
    dt = 0
    if activation:   
        if forceCodeCarbon:
            esternProcess.callCodeCarbone(x_predictor, y_response)
        elif operatingSystem == "Windows":
            VTuneSelectedDataset = datasets.index(name_csv) + 1
            print("chiama VTune Profiler")
            dt = esternProcess.VTuneProfilerInterface(VTuneSelectedDataset)
        else:
            esternProcess.callCodeCarbone(x_predictor, y_response)
    return dt



    