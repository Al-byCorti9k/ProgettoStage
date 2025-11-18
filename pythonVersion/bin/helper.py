#implementazione della personalizzazione degli output
#da linea di comando.
import argparse

parser = argparse.ArgumentParser(prog='benchmarkOfSMLM',
                                 description = "programma per il calcolo della LOOCV, MCC, consumi energetici di un modello di regressione lineare",
                                exit_on_error=False, suggest_on_error=True )
parser.add_argument('-i', nargs = '+', 
                    help = "choose datasets based on association list", type = int)
parser.add_argument('-al', nargs = '?', 
                    help = "shows the association list")
parser.add_argument('-od', nargs ='?', 
                    help = "select the saving path of results. Defult is file directory")
parser.add_argument('-cn', nargs = '?', 
                    help = "select the name of the categorical column to predict. Default is the last column")
parser.add_argument('-t', nargs = '?', help = "compute LOOCV's time")
parser.add_argument('-e', nargs = '?', help = "compute energy consumption")
parser.add_argument('-b', nargs = '?', help = "compute the MCC")
args = parser.parse_args()
#print(args)
#ricorda, devi controllare che non ci siano valori ripetuti nella lista 
#dei dataset che vuoi usare, e che non venga scelta una colonna non 
#categorica.