import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import pathlib
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef
from sklearn.pipeline import make_pipeline, Pipeline
import dataprocess
from sklearn.impute import SimpleImputer
import helper

# Versione Python del classificatore binario con regressione lineare e LeaveOneOut cross validation
# per progetto di stage e tesi presso l'università Milano Bicocca

#lista delle opzioni scelte da linea di comando
args = parser.parse_args()


# ottengo il percorso del file Main.py
p = pathlib.Path(__file__)

# ottengo il percorso del file .csv
 
dtype_dict, name_csv = dataprocess.diabetes_type1_dset()

p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
                                                



df = pd.read_csv(p_csv, dtype = dtype_dict)
print(df.tail(5))

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





#ottengo tutte le colonne
x_predictor = df[df.columns[:-1]]
y_response = df[df.columns[-1]]


# uso la classificazione con regressione logistica e LOOCV 
model = linear_model.LogisticRegression(max_iter = 1000)
cvp = model_selection.LeaveOneOut()

# preprocessare i dati è fondamentale per rendere comparabili i valori 
# categorici con quelli numerici, i quali a loro volta vengono scalati 
# per permettere un confronto adeguato
preprocessor = ColumnTransformer(
    transformers=[
        ('num', numeric_trasformer, selector(dtype_exclude="category")),
        ('cat', categorical_trasformer, selector(dtype_include="category"))
    ]
)
# creo una pipeline che effettua il preprocessing e poi applica il modello
clf = make_pipeline(preprocessor, model)

# viene effettuata la LOOCV predittiva, in modo da ottenere 
# le previsioni di ciascun fold, per poi valutare la prestazione
# del modello con la metrica MCC

y_predict = model_selection.cross_val_predict(clf, x_predictor, y_response, cv = cvp )
time = model_selection.cross_validate(clf, x_predictor, y_response, cv = cvp)

#print(y_predict.shape) 
#print(y_response)
#print(y_response.value_counts(normalize=True))
#print(df.dtypes)

MCC = matthews_corrcoef(y_response, y_predict)
t = np.sum(time["fit_time"])
print("coefficiente MCC del classificatore: ", MCC)
print("tempo di esecuzione di LOOCV in secondi: "+ str(t) + " s")
print("tempo di esecuzione di LOOCV in ms: " + str(t * 1000) + " ms")




