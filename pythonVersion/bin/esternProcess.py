#classe per comunicazione con i moduli esterni. In particolare, con VTune Profiler e CodeCarbon (forse
# la sua fork) per il calcolo dei consumi energetici

import pathlib
from datetime import datetime
import subprocess
import ctypes, sys
import platform
import csv

now = datetime.now()
now_str = now.strftime("%Y-%m-%d_%Hh-%Mm-%Ss")



p = pathlib.Path(__file__)
p_csv = pathlib.PurePath(p).parents[1].joinpath("results",now_str)
exp_csv = f"experiment_{now_str}.csv"
f_csv = pathlib.PurePath(p).parents[1].joinpath("results",exp_csv)
path = pathlib.PurePath(p).parents[1]
result_path = rf"..\results\experiment_{now_str}"
collector = rf"vtune -collect system-overview -no-analyze-system -r {p_csv} -knob analyze-power-usage=true -- python Main.py"
converter_to_csv = rf"vtune -report summary -result-dir {p_csv} -report-output {f_csv} -format csv -csv-delimiter comma"

# questa funzione serve a lanciare i comandi sulla shell con i permessi elevati. E' con 
# questa funzione che interagiremo con VTune Profiler attraverso i suoi comandi CLI
def run_command(cmd):
    process = subprocess.Popen(
         cmd,
         shell=True,
         stdout=subprocess.PIPE,
         stderr=subprocess.PIPE,
         universal_newlines=True,
         cwd=path
         )
    process.communicate()
    
# questa funzione verifica se l'utente ha i permessi di admin. In caso contrario, rilancia il programma
# da capo con i permessi elevati.
def is_admin():
    try:
        return ctypes.windll.shell32.IsUserAnAdmin()
    except:
        return False

flag =pathlib.Path(__file__).parent / "child_done.flag"
time = pathlib.Path(__file__).parent / "time_time.flag"
output = p.parents[1].joinpath("results",exp_csv)


    # viene controllato se si è admin e si eseguono i comandi. Poi se output, cioè il report csv 
    # che dovrebbe aver generato VTune Profiler esiste, aggiorna opportunamente l'exit_code
def VTuneProfilerInterface(dataset):
	if is_admin():
					args = f" -i {dataset}"
					print("inizia la collezione dei dati, attendi qualche minuto...\n")
					run_command(collector + args)
					print("\n La collezione dei dati è conclusa! Inizia la conversione in formato csv...\n")
					run_command(converter_to_csv)

					if output.exists():
									exit_code = 0
					else:
									exit_code = 1
					
					# vengono scritti due file, che serviranno al processo padre per verificare 
					# lo status del figlio. Purtroppo Windows non permette con shell32.shellExcuteW
					# le normali gestioni dei processi padre/figlio come si potrebbe fare su Linux.
					# Questo è il modo più semplice che ho trovato.

					flag.write_text(str(exit_code))
					time.write_text(str(now_str))
					
					sys.exit(exit_code)
	else:
					# deve rilanciare il programma: lo rilancia senza i permessi da admin
					#script_path = pathlib.PurePath(p).parents[1].joinpath("results", "Main.py")
					#args = f'"{script_path}" -i "{dataset}"'
					print("ciao prova")
					ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, " ".join(sys.argv), None, 1)
					#ctypes.windll.shell32.ShellExecuteW(None, "runas", sys.executable, " ".join(sys.argv), None, 1)

	# quando il processo padre chiama il "figlio" si mette in attesa finchè i report flag e time
	# non vengono generati. Si occupa di controllarne lo status
	while True:
					if flag.exists():
									exit_code = int(flag.read_text())
									print("Exit code:", exit_code)
									flag.unlink()

									break


	
	# precedentemente abbiamo creato due file, time e flag. Il primo per memorizzare il momento dell'esecuzione
	# dell'esperimento, il secondo per memorizzare l'esito.
	# Questa sezione usa time per poter risalire al file csv creato dai comandi VTune Profiler sopra
	# per poter poi risalire all'informazione del consumo energetico
	energyConsumption = []
	if time.exists():
					text = time.read_text()
					time.unlink()
	f_csv = pathlib.PurePath(p).parents[1].joinpath("results",f"experiment_{text}.csv")

	# viene aperto il file csv e si controlla riga per riga se vi è presente il termine "Package_0"
	# Soluzione poco elegante, ma VTune Profiler genera sempre in questo modo i csv, e dato che il 
	# formato non è facilmente importabile in un dataframe Pandas, sembra essere la soluzione più veloce
	with open(f_csv,'rt') as f:
					data = csv.reader(f)
					for row in data:
									if any("Package_0" in cell for cell in row):
										energyConsumption.append(row)
										

	print(f"il consumo energetico in mJ è: {energyConsumption[1][2]} mJ")
	print("fine del programma!!")
	return energyConsumption[1][2]


# funzione che controlla il sistema operativo su cui si sta eseguendo il codice
def checkOperatingSystem():
    os = platform.system()
    return os