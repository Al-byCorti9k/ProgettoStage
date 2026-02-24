# programma che calcola i consumi energetici di un programma GO attraverso chiamate con CodeCarbon alla RAPL interface

import pathlib
import argparse
import subprocess
import pandas as pd
from codecarbon import OfflineEmissionsTracker

def get_latest_experiment_file(directory):
    """
    Trova il file experiment_rust_*.csv più recente in base alla data di modifica.
    Usato per identificare il file appena creato dall'eseguibile Go.
    """
    files = list(directory.glob("experiment_go_*.csv"))
    if not files:
        return None
    # Restituisce il file con la data di modifica più recente
    return max(files, key=lambda x: x.stat().st_mtime)

def run_experiment():
    # --- PARSING DEGLI ARGOMENTI ---
    parser = argparse.ArgumentParser(description="Runner per programma Go con monitoraggio CodeCarbon")
    # -d accetta zero o più valori (es. -d 2 3)
    parser.add_argument('-d', nargs='*', type=str, help='Vettore numerico')
    # -t accetta zero o più valori (es. -t "Male (1=Yes,0=No)" "Exitus")
    parser.add_argument('-t', nargs='*', type=str, help='Vettore di stringhe')
    
    args = parser.parse_args()
    d_vector = args.d if args.d is not None else []
    t_vector = args.t if args.t is not None else []

    # Verifica che se vengono forniti target, il loro numero sia uguale ai dataset
    if len(t_vector) > 0 and len(d_vector) != len(t_vector):
        print("Errore: I vettori -d e -t devono avere la stessa lunghezza.")
        return

    # --- PREPARAZIONE DELLE CARTELLE DI OUTPUT ---
    p = pathlib.Path(__file__)                     # percorso di questo script
    output_dir = p.parents[1] / "results"          # risale di due livelli e va in /results
    output_dir.mkdir(parents=True, exist_ok=True)  # crea la directory se non esiste
    csv_carbon = output_dir / "emissions.csv"      # file temporaneo generato da CodeCarbon

    # Il numero di iterazioni è pari al numero di dataset, oppure 1 se non ce ne sono
    iterations = max(len(d_vector), 1)
    # Lista che conterrà i percorsi dei file generati in questa esecuzione (per il merge finale)
    session_files = []

    # --- CICLO PRINCIPALE: UNA ITERAZIONE PER OGNI COPPIA DATASET/TARGET ---
    for i in range(iterations):
        print(f"\n--- Inizio Iterazione {i+1}/{iterations} ---")
        
        # Costruzione della riga di comando per l'eseguibile Rust
        # L'eseguibile si trova nella stessa cartella di questo script, con nome goVersion.exe
        cmd_list = [str(p.parents[1] / "src" / "goVersion.exe")]
        if d_vector:
            cmd_list.extend(["-d", d_vector[i]])   # aggiunge il flag -d con il suo valore
        if t_vector:
            cmd_list.extend(["-t", t_vector[i]])   # aggiunge il flag -t con il suo valore

        # --- CONFIGURAZIONE DEL TRACKER CODECARBON ---
        tracker = OfflineEmissionsTracker(
            country_iso_code="ITA",          # codice ISO del paese (Italia)
            output_dir=str(output_dir),      # dove salvare emissions.csv
            measure_power_secs=1,            # intervallo di misurazione (1 secondo)
            tracking_mode="process",          # traccia solo questo processo
            on_csv_write="append"             # appende i dati al file CSV esistente
        )
        
        # Avvio del monitoraggio energetico
        tracker.start()
        try:
            # Esecuzione dell'eseguibile Rust con gli argomenti costruiti
            subprocess.run(cmd_list, check=True)
        except Exception as e:
            print(f"Errore durante l'esecuzione di Go: {e}")
        finally:
            # Arresto del tracker (salva i dati di consumo nel file emissions.csv)
            tracker.stop()

        # --- POST-ELABORAZIONE: AGGIUNTA DEL CONSUMO AL FILE GENERATO DA GO ---
        # Verifica che il file emissions.csv sia stato creato
        if csv_carbon.exists():
            # Legge il file CSV delle emissioni
            df_carbon = pd.read_csv(csv_carbon)
            # Prende l'ultimo valore della colonna 'energy_consumed' (consumo in kWh)
            ultimo_consumo = df_carbon.iloc[-1]['energy_consumed']            
            # Trova il file experiment_rust_*.csv più recente (quello appena creato da Go)
            target_file = get_latest_experiment_file(output_dir)
            
            # Se il file esiste e non è già stato processato in questa sessione
            if target_file and target_file not in session_files:
                session_files.append(target_file)          # lo aggiunge alla lista per il merge
                df_exp = pd.read_csv(target_file)          # legge il file generato da Go
                
                # Aggiunge una nuova colonna "energy consumption (kWh)" con il valore di consumo
                # Se il file contiene più righe, assegna lo stesso valore a tutte
                df_exp["energy consumption (kWh)"] = ultimo_consumo
                
                # Sovrascrive il file con i dati arricchiti
                df_exp.to_csv(target_file, index=False)
                print(f"\nDato ({ultimo_consumo} kWh) inserito in: {target_file.name}")
            
            # Elimina il file emissions.csv per non accumulare dati di iterazioni precedenti
            csv_carbon.unlink()

    # --- FASE DI FUSIONE (MERGE) DEI FILE GENERATI ---
    # Se sono stati generati più di un file (cioè più iterazioni con output distinti)
    if len(session_files) > 1:
        print(f"\n--- Fusione di {len(session_files)} file in corso ---")
        
        # Carica il primo file della sessione (il più vecchio, secondo l'ordine di esecuzione)
        main_df = pd.read_csv(session_files[0])
        
        # Per ogni file successivo, lo concatena al DataFrame principale
        for extra_file in session_files[1:]:
            temp_df = pd.read_csv(extra_file)
            main_df = pd.concat([main_df, temp_df], ignore_index=True)
            # Elimina il file ora ridondante (dopo la fusione)
            extra_file.unlink()
            print(f"File {extra_file.name} fuso e rimosso.")
        
        # Salva il DataFrame unificato nel primo file della sessione
        main_df.to_csv(session_files[0], index=False)
        print(f"Risultato finale salvato in: {session_files[0].name}")
    else:
        # Caso in cui c'è un solo file (o nessuno): non serve fare merge
        print("\nNessuna fusione necessaria (singolo file o nessun file generato).")

if __name__ == "__main__":
    run_experiment()
