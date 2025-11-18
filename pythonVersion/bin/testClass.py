import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import pathlib
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef
from sklearn.pipeline import make_pipeline
from enum import Enum
import argparse

# Versione Python del classificatore binario con regressione lineare e LeaveOneOut cross validation
# per progetto di stage e tesi presso l'università Milano Bicocca

# ottengo il percorso del file Main.py
p = pathlib.Path(__file__)

# ottengo il percorso del file .csv
name_csv = "Takashi2019_diabetes_type1_dataset_preprocessed.csv"
p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
df = pd.read_csv(p_csv)

#print(df.dtypes)

'''
parser = argparse.ArgumentParser(prog='benchmarkOfSMLM',
                                 description = "programma per il calcolo della LOOCV, MCC, consumi energetici di un modello di regressione lineare",
                                exit_on_error=False, suggest_on_error=True )
parser.add_argument('-i', nargs = '+', help = "choose datasets based on association list", type = int)
parser.add_argument('-al', nargs = '?', help = "shows the association list")
parser.add_argument('-o', nargs ='?', help = "choose the saving path of results. Defult is file directory")
args = parser.parse_args()
#restituisce gli input presi da linea di comando
print(args.i, args.o)
#parser.print_help()
'''
indice = "pino"
data = {
    indice : 2,
    "daniele" : 3,
    "l'africano" : 4
}

print(data[indice])

lista = ["cane", "gatto"]
print(lista[1])
'''
class Animal(Enum):
    ant = 1
    bee = 2
    cat = 3

print(Animal.cat.value == 3)

for animal in Animal:
    if animal.value == 2:
        print("ciao bee")

guard = True
while(guard):
    try:
        inputs = input("inserisci un numero: ")
        lang = int(inputs)
    except  Exception:
        print("il carattere che hai inserito, \""+ inputs + "\", non è un numero!")
    else:
        guard = False    


match lang:
    case Animal.ant.value:
        print("ciao formica")
    case Animal.bee.value:
        print("ciao apetta")
    case Animal.cat.value:
        print("ciao micino")
    case _:
        print("vaffambrodo")

'''