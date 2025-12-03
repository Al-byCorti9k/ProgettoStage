import pandas as pd
import numpy as np
import pathlib
from sklearn.metrics import matthews_corrcoef
import dataprocess
import viewer
import esternProcess



#ottengo la lista di argomenti da linea di comando
args = viewer.parser.parse_args()
viewer.showAssociationList(args.al)


#gestione casistica scelta multipla dei datasets. ottengo delle liste
#dtype_dict, name_csv = helper.multipleDatasetSelection(args.i)
dtype_csv_dict = viewer.multipleDatasetSelection(args.i)

for key, value in dtype_csv_dict.items():
    dtype_dict = value
    name_csv = key 
    # ottengo il percorso del file .csv
    p = pathlib.Path(__file__)
    p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
    df = pd.read_csv(p_csv, dtype = dtype_dict)
    viewer.datasetPreview(df, name_csv)
    #ottengo tutte le colonne

    x_predictor, y_response, columnNotExist, columnNonCat  = dataprocess.columnPredictionSelect(args.cn, df)
   
   # Se l'utente ha selezionato una colonna non esistente, non categorica oppure
   # è stata selezionata la modalità visualizzazione, si procede nell'iterazione
    if columnNotExist or columnNonCat or args.v :
        continue
    y_predict, time = dataprocess.Logistic_Regression_Validation(x_predictor, y_response)
    
    os = esternProcess.checkOperatingSystem()
    consumptions = dataprocess.energyConsumption(os, args.e, args.ec, name_csv, x_predictor, y_response)

    MCC = matthews_corrcoef(y_response, y_predict)
    

    dataprocess.addRowToCSV(MCC, time, consumptions, os, args.e, name_csv, args.ec)

# Stampa e salvataggio dei risultati
dataprocess.createCSV(args.r)   
viewer.visualizeResults(args.v)




