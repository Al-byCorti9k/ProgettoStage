import pandas as pd
import numpy as np
import pathlib
#from sklearn import linear_model, model_selection 
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
   
   
    if columnNotExist or columnNonCat or args.v :
        continue
    y_predict, time = dataprocess.Logistic_Regression_Validation(x_predictor, y_response)
    
    os = esternProcess.checkOperatingSystem()
    consumptions = dataprocess.energyConsumption(os, args.e, args.ec, name_csv, x_predictor, y_response)

    MCC = matthews_corrcoef(y_response, y_predict)
   # t = np.sum(time["fit_time"])
    t = time
    print("coefficiente MCC del classificatore: ", MCC)
    print("tempo di esecuzione di LOOCV in secondi: "+ str(t) + " s")
    print("tempo di esecuzione di LOOCV in ms: " + str(t * 1000) + " ms")
    if consumptions != 0:
        print(f"il consumo energetico in mJ è: {consumptions} mJ")




