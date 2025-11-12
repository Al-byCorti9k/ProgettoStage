import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import pathlib
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef
from sklearn.pipeline import make_pipeline


# Versione Python del classificatore binario con regressione lineare e LeaveOneOut cross validation
# per progetto di stage e tesi presso l'università Milano Bicocca

# ottengo il percorso del file Main.py
p = pathlib.Path(__file__)

# ottengo il percorso del file .csv
name_csv = "100_7717_peerj_5665_dataYM2018_neuroblastoma.csv"
p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
                                                


# prima di importare il dataset, mi assicuro che le colonne vengano abbiano 
# il tipo corretto. La maggior parte dei dati sono "categorici", 
# ossia i valori appartengono a delle classi.
cat_cols = ['age', 'sex', 'site', 'stage', 'risk',
       'autologous_stem_cell_transplantation', 'radiation',
       'degree_of_differentiation', 'UH_or_FH', 'MYCN_status ',
       'surgical_methods', 'outcome']
dtype_dict = {col: "category" for col in cat_cols}

# importo il dataset in un oggetto dataframe
df = pd.read_csv(p_csv, dtype = dtype_dict)
print(df.tail(5))

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
        ('num', StandardScaler(), selector(dtype_exclude="category")),
        ('cat', OneHotEncoder(), selector(dtype_include="category"))
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
# MCC = 0.4512987012987013



