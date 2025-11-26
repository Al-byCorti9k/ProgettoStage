
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
import os

# Versione Python del classificatore binario con regressione lineare e LeaveOneOut cross validation
# per progetto di stage e tesi presso l'università Milano Bicocca

# ottengo il percorso del file Main.py
p = pathlib.Path(__file__)
'''
# ottengo il percorso del file .csv
name_csv = "Takashi2019_diabetes_type1_dataset_preprocessed.csv"
p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
df = pd.read_csv(p_csv)
'''
#print(df.dtypes)


parser = argparse.ArgumentParser(prog='benchmarkOfSMLM',
                                 description = "programma per il calcolo della LOOCV, MCC, consumi energetici di un modello di regressione lineare",
                                exit_on_error=False, suggest_on_error=True )
parser.add_argument('-i', nargs = '+', help = "choose datasets based on association list", type = int)
parser.add_argument('-al', action = "store_true", help = "shows the association list")
parser.add_argument('-o', nargs ='?', help = "choose the saving path of results. Defult is file directory")
args = parser.parse_args()
#restituisce gli input presi da linea di comando
print(args)
#parser.print_help()

indice = "pino"
data = {
    indice : 2,
    "daniele" : 3,
    "l'africano" : 4
}



lista = ["cane", "gatto"]
print(lista[1])
pippo = None
if pippo == None:
    print("sabaku")


from datetime import datetime

now = datetime.now()
now_str = now.strftime("%Y-%m-%d_%Hh-%Mm-%Ss")
print(now_str)

import subprocess
p = pathlib.Path(__file__)
p_csv = pathlib.PurePath(p).parents[1].joinpath("results",now_str)
exp_csv = f"experiment_{now_str}.csv"
f_csv = pathlib.PurePath(p).parents[1].joinpath("results",exp_csv)
path = pathlib.PurePath(p).parents[1]
result_path = rf"..\results\experiment_{now_str}"
collector = rf"vtune -collect system-overview -r {p_csv} -knob analyze-power-usage=true -- python Main.py"
converter_to_csv = rf"vtune -report summary -result-dir {p_csv} -report-output {f_csv} -format csv -csv-delimiter comma"






#TODO provare a fare un esempio prototipo per fare il test che dicevi.

# Source - https://stackoverflow.com/a
# Posted by Martín De la Fuente, modified by community. See post 'Timeline' for change history
# Retrieved 2025-11-26, License - CC BY-SA 4.0

def run_command(cmd):
    process = subprocess.Popen(
         cmd,
         shell=True,
         stdout=subprocess.PIPE,
         stderr=subprocess.PIPE,
         universal_newlines=True,
         cwd=path
         )
    stdout, stderr = process.communicate()
    return process.returncode, stdout, stderr


import ctypes, sys

def is_admin():
    try:
        return ctypes.windll.shell32.IsUserAnAdmin()
    except:
        return False

if is_admin():
    run_command(collector)
    returncode, stdout, stderr = run_command(converter_to_csv)
    if returncode != 0:
        print(f"error listing directory: {stderr}")
    print(returncode)
else:
    # Re-run the program with admin rights
    ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, " ".join(sys.argv), None, 1)








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
    case Animal.bee.value:clear
        print("ciao apetta")
    case Animal.cat.value:
        print("ciao micino")
    case _:
        print("vaffambrodo")

'''
