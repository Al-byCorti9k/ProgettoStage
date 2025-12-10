#classe per comunicazione con i moduli esterni. In particolare, con VTune Profiler e CodeCarbon (forse
# la sua fork) per il calcolo dei consumi energetici

import pathlib
from datetime import datetime
import subprocess
import ctypes
import sys
import csv
import shutil

import pandas as pd
from codecarbon import OfflineEmissionsTracker

import dataprocess


now = datetime.now()
now_str = now.strftime("%Y-%m-%d_%Hh-%Mm-%Ss")

p = pathlib.Path(__file__)
p_csv = pathlib.Path(p).parents[1] / "results" / now_str
exp_csv = f"experiment_{now_str}.csv"
f_csv = pathlib.Path(p).parents[1] / "results" / exp_csv
main_path = pathlib.PurePath(p).parents[1] / "bin"
<<<<<<< HEAD
cmd_path = rf"call C:\Users\utente\miniconda3\condabin\conda.bat activate stageENV"
=======


# funzione per selezionare il comando corretto per comunicare con VTune Profiler. Se si è in un ambiente conda,
# fa in modo che venga attivato quando il processo elevato chiama VTune Profiler, altrimenti esegue in ambiente python
def condaPathSelect(p_csv):
	
	conda = dataprocess.readConda()
	
	if conda != "notConda":
		cmd_path = rf"call C:\Users\utente\miniconda3\condabin\conda.bat activate {conda}"
	else:
		cmd_path = rf"echo nothing"
	collector = rf"{cmd_path} && vtune -collect system-overview -data-limit=200 -d=15 -discard-raw-data -finalization-mode=none -r {p_csv} -knob analyze-power-usage=true -knob sampling-interval=1000 -- python Main.py"

	return collector


>>>>>>> experimental
result_path = rf"..\results\experiment_{now_str}"
converter_to_csv = rf"vtune -report summary -result-dir {p_csv} -report-output {f_csv} -format csv -csv-delimiter comma"

# questa funzione serve a lanciare i comandi sulla shell con i permessi elevati. E' con 
# questa funzione che interagiremo con VTune Profiler attraverso i suoi comandi CLI
def run_command(cmd):
	process=subprocess.Popen(
	cmd,
	shell = True,
	stdout = subprocess.PIPE,
	stderr = subprocess.PIPE,
	universal_newlines = True,
	cwd = main_path
	)
<<<<<<< HEAD
	
	return process.communicate()

	
	
=======
	while process.poll() is None:
		line = process.stderr.readline()
		if not line:
			break
		print("VTune's Call:", line.rstrip())
	
	stdoutFull = process.stdout.read()

	return stdoutFull

>>>>>>> experimental
    
# questa funzione verifica se l'utente ha i permessi di admin. In caso contrario, rilancia il programma
# da capo con i permessi elevati.
def is_admin():
    try:
        return ctypes.windll.shell32.IsUserAnAdmin()
    except:
        return False


# funzione che pulisce le cartelle e i file creati da VTune Profiler

def cleaner(path):
	folder = path
	for item in folder.iterdir():
		# se il nome **non inizia per 'result'**
		if not item.name.startswith("result"):
			if item.is_file() or item.is_symlink():
				item.unlink()  # cancella file o symlink
			elif item.is_dir():
				shutil.rmtree(item)  # cancella cartelle ricorsivamente





# funzione che serve per controllare i diversi exit code delle chiamate con subprocess.Popen e
# verificare che tutte le operazioni si siano svolte come previsto
def flagExitCodeChecker():
		lines = flag.read_text().splitlines()
		lineIndex = 1
		
		for line in lines[:4]:
			lineF = line.rstrip()
			if lineF[-1:] != "0" and lineF[-1:] != ":":
				
				if  lineIndex == 1:
					print("il csv non è stato creato\n")
					
				if lineIndex == 2:
					print("La collezione non è andata a buon fine\n")
					print(line)
				if lineIndex == 3:
					print("il report non è andato a buon fine\n")
					print(line)
				
			lineIndex += 1
		
			
		flag.unlink()






flag =p.parent / "child_done.flag"
times = p.parent / "time_time.flag"
output = p.parents[1].joinpath("results",exp_csv)


# comandi a Intel VTune Profiler
def newProcessCommands(dataset):
	
<<<<<<< HEAD
	out0, error0 = run_command("conda activate stageENV")
	args = f" -i {dataset} -e --elevated"
	print("inizia la collezione dei dati, attendi qualche minuto...\n")
	out1, error1 = run_command(collector + args)
	print("\n La collezione dei dati è conclusa! Inizia la conversione in formato csv...\n")
	
	out2, error2 = run_command(converter_to_csv)
	
=======
	args = f" -i {dataset} -e --elevated"
	print("inizia la collezione dei dati, attendi qualche minuto...\n")
>>>>>>> experimental

	strderrCollector = run_command(condaPathSelect(p_csv) + args)
	
	print("\n La collezione dei dati è conclusa! Inizia la conversione in formato csv...\n")
	stderrConverter= run_command(converter_to_csv)
	
	if output.exists():
					exit_code = 0
	else:
					exit_code = 1
	# per la stampa e il controllo di tutti gli esiti delle operazioni
	ex0 = "result of CSV's creation: " + str(exit_code)
	ex1 = "result of Collection: " + str(strderrCollector)
	ex2 = "result of report: " + str(stderrConverter)
	allExitCode = f"{ex0}\n{ex1}\n{ex2}\n"
	flag.write_text(allExitCode)
	times.write_text(str(now_str))
	
	
	sys.exit(exit_code)




# funzione che ottiene dal CSV il dato del consumo energetico
def getEnergyFromCSV(f_csv):
	energyConsumption = []
	# viene aperto il file csv e si controlla riga per riga se vi è presente il termine "Package_0"
	with open(f_csv,'rt') as f:
					data = csv.reader(f)
					for row in data:
								if any("Package_0" in cell for cell in row):
										energyConsumption.append(row)
	cleaner(p.parents[1] / "results")	
	return energyConsumption




# Si interfaccia a Vtune Profiler e restituisce il consumo energetico in mJ
def VTuneProfilerInterface(dataset):
	if is_admin():
					newProcessCommands(dataset)
	else:
		cmd_line = [sys.argv[0], sys.argv[1], str(dataset),"--elevated","--elevatedloocv"]
		# rilancia il programma in una shell con i permessi di admin
		ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, " ".join(cmd_line), None, 1)

# quando il processo padre chiama il "figlio" si mette in attesa finchè i report flag e time
# non vengono generati. Si occupa di controllarne lo status
	while True:
			if flag.exists():
				flagExitCodeChecker()
				break
# precedentemente abbiamo creato due file, time e flag. Il primo per memorizzare il momento dell'esecuzione
# dell'esperimento, il secondo per memorizzare l'esito.
	if times.exists():
					text = times.read_text()
					times.unlink()
	f_csv = pathlib.PurePath(p).parents[1].joinpath("results",f"experiment_{text}.csv")

	energyConsumption = getEnergyFromCSV(f_csv)
	try:
		stimatedEnergy = energyConsumption[1][2]
	except Exception:
		print("VTune Profiler non è riuscito ad ottenere i consumi energetici")
		stimatedEnergy = 0
	return stimatedEnergy






#funzione che chiama il tracker del modulo CodeCarbon
indice = 0

def callCodeCarbone(x_predictor, y_response):
	tracker = OfflineEmissionsTracker(country_iso_code="ITA",
								  output_dir = p.parents[1] / "results",
								  measure_power_secs = 1,
								  tracking_mode = "process",
								   on_csv_write = "append"
									)
	tracker.start()
	dataprocess.Logistic_Regression_Validation(x_predictor, y_response)
	tracker.stop()

	df = pd.read_csv(f"{p.parents[1]}/results/emissions.csv")

	dt = df['cpu_energy'][indice]
	
	cleaner(p.parents[1] / "results")	
	return dt
