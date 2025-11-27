#implementazione della personalizzazione degli output
#da linea di comando.
import argparse
from enum import Enum
import dataprocess
import logging
import pandas as pd
from IPython.display import display

parser = argparse.ArgumentParser(prog='benchmarkOfSMLM',
                                 description = "programma per il calcolo della LOOCV, MCC, consumi energetici di un modello di regressione lineare",
                                exit_on_error=False, suggest_on_error=True )
parser.add_argument('-i', nargs = '+', 
                    help = "choose datasets based on association list", type = int)
parser.add_argument('-al', action = 'store_true', 
                    help = "shows the association list and exit")
parser.add_argument('-od', nargs ='?', 
                    help = "select the saving path of results. Defult is file directory")
parser.add_argument('-cn', nargs = '?', 
                    help = "select the name of the categorical column to predict. Default is the last column")
parser.add_argument('-t', action = 'store_false', help = "deactivate computation of LOOCV's time")
parser.add_argument('-e', action = 'store_false', help = "deactivate computation of energy consumption")
parser.add_argument('-ec', action="store_true", help = "force computation of energy consumption with codeCarbon. Default False")
parser.add_argument('-b', action = 'store_false', help = "deactivate computation of the MCC")
parser.add_argument('-v', action = 'store_true', help = "visualize a preview of selected datasets" )
#args = parser.parse_args()
#print(args)

#ricorda, devi controllare che non ci siano valori ripetuti nella lista 
#dei dataset che vuoi usare, e che non venga scelta una colonna non 
#categorica.

class Dataset(Enum):
    SEPSIS = 1
    NEUROBLASTOMA = 2
    DEPRESSION_HEART = 3
    CARDIAC_ARREST = 4
    DIABETES = 5

#gestione della scelta dei database
def multipleDatasetSelection(args):
    result_dict = {}
    if args == None:
       dtype_dict, name_csv =  dataprocess.datasetsSelection(Dataset.NEUROBLASTOMA.value)
       result_dict.update({name_csv : dtype_dict})
       
    else:
        #rimuovo i duplicati
        args = list(set(args))
        for userSelection in args:
            if userSelection >= 1 and userSelection <=5:
                dtype_dict, name_csv = dataprocess.datasetsSelection(userSelection)
                result_dict.update({name_csv : dtype_dict})
            else:
                print("il numero che hai inserito  \""+str(userSelection)+"\" non corrisponde a nessun dataset")
    return result_dict

def showAssociationList(userCheck):
    if userCheck:
        print("\nlist of available datasets\n")
        print("dataset "+"{:>20}".format("identifier\n"))
        for datasets in Dataset:
            space = 20 - len(datasets.name)
            print(datasets.name+":"+"{:>{}}".format(str(datasets.value), space)) 
        exit("\n")

#stampa una preview delle ultime 5 righe del dataset
def datasetPreview(dataset, name_csv):
    position = dataprocess.datasets.index(name_csv) + 1
    pd.set_option('display.max_columns', 20)
    pd.set_option('display.width', 310)
    infoMessage = "\nSelected dataset:\t"+Dataset(position).name+"\n"
    print(infoMessage)
    display(dataset.tail(5))
    print("\n")


