import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import pathlib
from sklearn import linear_model, model_selection
from sklearn.metrics import matthews_corrcoef

import array
#ottengo il percorso del file Main.py
p = pathlib.Path(__file__)

#ottengo il percorso del file .csv
p_csv = pathlib.PurePath(p).parents[2].joinpath("data", "100_7717_peerj_5665_dataYM2018_neuroblastoma.csv")

#importo il dataset in un oggetto dataframe
df = pd.read_csv(p_csv)
print(df.tail(5))

#ottengo tutte le colonne
x_predictor = df[df.columns[:-1]]
#print(x_predictor.shape)
y_response = df[df.columns[-1]]
#print(y_response.shape)


#sezione aggiornata

#obiettivo: leave-one-out cross validation per verificare che il modello
#abbia una capacità di generalizzazione solida. 



#importiamo il modello sulla quale si baserà l'algoritmo di machine learning
#NOTA al momento uso la logisticregression, di modo che i dati siano binari.
# questo ha dei problemi di performance perchè l'algoritmo potrebbe non finire 
# ben precisato di passi. 
model = linear_model.LogisticRegression(max_iter = 1000)

#indichiamo il metodo che useremo per validare le capacità di generalizzazione
cvp = model_selection.LeaveOneOut()

#possiamo avviare la validazione. Ho scelto la funzione cross_val_predict.
# rispetto alle altre due alternative (cross_val, cross_val_score) mi permette
# di ricavare comodamente tutti i valori delle predizioni nei singoli fold.
# tornerà utile per il calcolo dell'MCC

y_predict = model_selection.cross_val_predict(model, x_predictor, y_response, cv = cvp )
print(y_predict)

MCC = matthews_corrcoef(y_response, y_predict)
print(MCC)

# MCC = 0.46382761282805135 bassa correlazione