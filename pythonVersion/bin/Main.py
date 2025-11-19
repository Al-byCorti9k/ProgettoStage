import pandas as pd
import numpy as np
import pathlib
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef
import dataprocess
import helper



#ottengo la lista di argomenti da linea di comando
args = helper.parser.parse_args()
helper.showAssociationList(args.al)


#gestione casistica scelta multipla dei datasets. ottengo delle liste
#dtype_dict, name_csv = helper.multipleDatasetSelection(args.i)
#TODO FARE IN MODO CHE NON CI SIANO ELEMENTI RIPETUTI IN ARGS.I
dtype_csv_dict = helper.multipleDatasetSelection(args.i)

for key, value in dtype_csv_dict.items():
    dtype_dict = value
    name_csv = key 
    # ottengo il percorso del file .csv
    p = pathlib.Path(__file__)
    p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
    df = pd.read_csv(p_csv, dtype = dtype_dict)
    print(df.tail(5))
    #ottengo tutte le colonne
    x_predictor, y_response = dataprocess.columnPredictionSelect(args.cn, df)


    # uso la classificazione con regressione logistica e LOOCV 
    model = linear_model.LogisticRegression(max_iter = 1000)
    cvp = model_selection.LeaveOneOut()

    # preprocessare i dati è fondamentale per rendere comparabili i valori 
    # categorici con quelli numerici, i quali a loro volta vengono scalati 
    # per permettere un confronto adeguato

    # creo una pipeline che effettua il preprocessing e poi applica il modello
    clf = dataprocess.make_pipeline(dataprocess.preprocessor, model)

    # viene effettuata la LOOCV predittiva, in modo da ottenere 
    # le previsioni di ciascun fold, per poi valutare la prestazione
    # del modello con la metrica MCC

    y_predict = model_selection.cross_val_predict(clf, x_predictor, y_response, cv = cvp )
    time = model_selection.cross_validate(clf, x_predictor, y_response, cv = cvp)



    MCC = matthews_corrcoef(y_response, y_predict)
    t = np.sum(time["fit_time"])
    print("coefficiente MCC del classificatore: ", MCC)
    print("tempo di esecuzione di LOOCV in secondi: "+ str(t) + " s")
    print("tempo di esecuzione di LOOCV in ms: " + str(t * 1000) + " ms")




