# punto di accesso al programma per il calcolo della LOOCV di un modello di regressione lineare
# e dei relativi consumi energetici con CodeCarbon e Intel VTune Profiler

import pathlib

import pandas as pd
from sklearn.metrics import matthews_corrcoef
import numpy as np

import dataprocess
import viewer
import sys
import subprocess

def main():
    #ottengo la lista di argomenti da linea di comando
    args = viewer.parser.parse_args()
    viewer.showAssociationList(args.al)

    # Prendiamo il nome del conda enviroment
    if not args.elevated and args.e:
        if not args.ec:
            dataprocess.setConda()

    if not args.elevated:
         installdependencies()




    #gestione casistica scelta multipla dei datasets. ottengo delle liste
    dtype_csv_dict = viewer.multipleDatasetSelection(args.i)


    for key, value in dtype_csv_dict.items():
        dtype_dict = value
        name_csv = key 
        # ottengo il percorso del file .csv
        p = pathlib.Path(__file__)
        p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
        df = pd.read_csv(p_csv, dtype = dtype_dict)
        if not args.elevated:
            viewer.datasetPreview(df, name_csv)
        #ottengo tutte le colonne

        x_predictor, y_response, columnNotExist, columnNonCat  = dataprocess.   columnPredictionSelect(args.cn, df)
    
    # Se l'utente ha selezionato una colonna non esistente, non categorica oppure
    # è stata selezionata la modalità visualizzazione, si procede nell'iterazione
        if columnNotExist or columnNonCat or args.v :
            continue
        MCC = 0
        times = 0
        if not args.elevated:
        
            # preprocessing dei sample
            x_predictor = dataprocess.preprocessor.fit_transform(x_predictor)
            # preprocessing della colonna target
            y_response_filled = dataprocess.targetColumnFillNaN(y_response) 

            y_predict, times = dataprocess.regressionValidation(x_predictor, y_response_filled)
        
            MCC = matthews_corrcoef(y_response_filled, y_predict)
          

        os = dataprocess.checkOperatingSystem()
    
        consumptions = dataprocess.energyConsumption(os, args.e, args.ec, name_csv, x_predictor, y_response)
    
        dataprocess.addRowToCSV(consumptions, os, args.e, name_csv, args.ec, MCC, times, y_response.name)  
    
    


    # Stampa e salvataggio dei risultati
    if not args.elevated and not args.v:
  
        dataprocess.createCSV(args.r)   
        viewer.visualizeResults(args.v)


# gestisce le dipendenze; se mancano le installa
def installdependencies():
	# List of libraries to check/install
	libraries = ["codecarbon", "pandas", "scikit-learn", "ipython"]

	for lib in libraries:
		try:
			__import__(lib)
			print(f"{lib} is already installed.")
		except ImportError:
			print(f"{lib} not found. Installing...")
			subprocess.check_call([sys.executable, "-m", "pip", "install", lib])


if __name__ == "__main__":
    
    main()