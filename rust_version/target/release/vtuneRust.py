# consiglio: eseguire sempre nel conda enviroment con le dipendenze richieste
import pandas as pd
import pathlib
import argparse
import csv
import shutil

def mj_to_kwh(energy_in_mj):
    """Converte MilliJoule in Kilowattora."""
    # 1 kWh = 3,600,000,000 mJ
    return (float(energy_in_mj) / (3.6 * 10**9))

def get_energy_from_vtune_csv(f_csv):
    """Estrae il valore energetico associato a Package_0 dal CSV di VTune."""
    energy = []
    
    try:
        with open(f_csv, 'rt', encoding='utf-8') as f:
            reader = csv.reader(f)
            for row in reader:
                if any("Package_0" in cell for cell in row):
                    energy.append(row)
                    
        # Verifichiamo che la lista contenga abbastanza elementi prima di accedere
        if len(energy) >= 2 and len(energy[1]) >= 3:
            return energy[1][2]
        else:
            print("VTune Profiler non è riuscito ad ottenere i consumi energetici")
            return 0.0

    except FileNotFoundError:
        print(f"Errore: Il file {f_csv} non è stato trovato.")
        return 0.0
    except (OSError, csv.Error) as e:
        print(f"Nota: Impossibile leggere dati da {f_csv}: {e}")
        return 0.0

def get_latest_experiment_file(directory):
    """Trova il file experiment_rust_...csv con la data di modifica più recente."""
    files = list(directory.glob("experiment_rust_*.csv"))
    if not files:
        return None
    return max(files, key=lambda x: x.stat().st_mtime)

def main():
    parser = argparse.ArgumentParser(description="Processore risultati VTune ed Energia")
    parser.add_argument('id', type=str, help='Il numero identificativo dell\'esperimento (*)')
    args = parser.parse_args()
    
    # --- GESTIONE PERCORSI ---
    current_file_path = pathlib.Path(__file__).resolve()
    base_dir = current_file_path.parent.parent.parent
    results_dir = base_dir / "results"

    # CREAZIONE CARTELLA RESULTS SE NON ESISTE
    results_dir.mkdir(parents=True, exist_ok=True)
    
    # -------------------------

    vtune_csv = results_dir / f"summary_report__d_{args.id}.csv"
    vtune_folder = results_dir / f"vtune_results__d_{args.id}"
    
    print(f"Percorso base rilevato: {base_dir}")
    print(f"Cartella risultati: {results_dir}")
    print(f"Elaborazione ID: {args.id}")

    # 1. Estrazione Energia
    raw_energy = 0.0
    if vtune_csv.exists():
        raw_energy = get_energy_from_vtune_csv(vtune_csv)
        print(f"Energia estratta: {raw_energy} mJ")
    else:
        print(f"Attenzione: File {vtune_csv.name} non trovato in {results_dir}.")
        return 

    # 2. Conversione
    energy_kwh = mj_to_kwh(raw_energy)
    
    # 3. Pulizia file e cartelle VTune
    if vtune_csv.exists():
        vtune_csv.unlink()
        print(f"Rimosso file: {vtune_csv.name}")
    
    if vtune_folder.exists() and vtune_folder.is_dir():
        shutil.rmtree(vtune_folder)
        print(f"Rimosso cartella: {vtune_folder.name}")

    # 4. Inserimento nel file esperimento più recente
    target_file = get_latest_experiment_file(results_dir)
    
    if target_file:
        try:
            df = pd.read_csv(target_file, index_col = False)
            print(df.tail(5))
            df["energy consumption (kWh)"] = energy_kwh
            df.to_csv(target_file, index=False)
            print(f"Successo: {energy_kwh:.10f} kWh inseriti in {target_file.name}")
        except Exception as e:
            print(f"Errore durante l'aggiornamento del file CSV: {e}")
    else:
        print(f"Errore: Nessun file 'experiment_rust_*.csv' trovato in {results_dir}")

if __name__ == "__main__":
    main()