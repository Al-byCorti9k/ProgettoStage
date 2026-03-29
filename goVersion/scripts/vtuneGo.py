# consiglio: eseguire sempre nel conda enviroment con le dipendenze richieste
import pandas as pd
import pathlib
import argparse
import csv
import shutil

def mj_to_kwh(energy_in_mj):
    # Converte MilliJoule in Kilowattora.
    # 1 kWh = 3,600,000,000 mJ
    return float(energy_in_mj) / (3.6 * 10**9)

def get_energy_from_vtune_csv(f_csv):
    # Estrae il valore energetico associato a Package_0 dal CSV di VTune.
    energy = []
    try:
        with open(f_csv, 'rt', encoding='utf-8') as f:
            reader = csv.reader(f)
            for row in reader:
                if any("Package_0" in cell for cell in row):
                    energy.append(row)
        if len(energy) >= 2 and len(energy[1]) >= 3:
            return energy[1][2]
        else:
            print("VTune Profiler failed to obtain energy consumption")
            return 0.0
    except FileNotFoundError:
        print(f"Error: file {f_csv} not found.")
        return 0.0
    except (OSError, csv.Error) as e:
        print(f"Warning: Unable to read data from {f_csv}: {e}")
        return 0.0

def get_latest_experiment_file(directory):
    # Trova il file experiment_go_...csv con la data di modifica più recente.
    files = list(directory.glob("experiment_go_*.csv")) 
    if not files:
        return None
    return max(files, key=lambda x: x.stat().st_mtime)

def main():
    parser = argparse.ArgumentParser(description="VTune and Energy results processor")
    parser.add_argument('id', type=str, help='The experiment ID number (*)')
    args = parser.parse_args()

    # --- GESTIONE PERCORSI CORRETTA ---
    # Lo script è in ./scripts/ ; risaliamo di due livelli per arrivare alla radice del progetto
    base_dir = pathlib.Path(__file__).parent.parent          # radice del progetto
    results_dir = base_dir / "results"                       # cartella results (allo stesso livello di scripts)

    # CREAZIONE CARTELLA RESULTS SE NON ESISTE
    results_dir.mkdir(parents=True, exist_ok=True)

    vtune_csv = results_dir / f"summary_report__d_{args.id}.csv"
    vtune_folder = results_dir / f"vtune_results__d_{args.id}"

    print(f"Base path: {base_dir}")
    print(f"Result path: {results_dir}")
    print(f"Processing ID: {args.id}")

    # Estrazione Energia
    raw_energy = 0.0
    if vtune_csv.exists():
        raw_energy = get_energy_from_vtune_csv(vtune_csv)
        print(f"Energy extracted: {raw_energy} mJ")
    else:
        print(f"Warning: File {vtune_csv.name} not found in {results_dir}.")
        return

    # Conversione
    energy_kwh = mj_to_kwh(raw_energy)

    # Pulizia file e cartelle VTune
    if vtune_csv.exists():
        vtune_csv.unlink()
        print(f"Removed file: {vtune_csv.name}")

    if vtune_folder.exists() and vtune_folder.is_dir():
        shutil.rmtree(vtune_folder)
        print(f"Removed folder: {vtune_folder.name}")

    # Inserimento nel file esperimento più recente
    target_file = get_latest_experiment_file(results_dir)

    if target_file:
        try:
            df = pd.read_csv(target_file, index_col = False)
            df["energy consumption (kWh)"] = energy_kwh
            df.to_csv(target_file, index=False)
            print(f"Success: {energy_kwh:.10f} kWh entered in {target_file.name}")
            # stampa il risultato
            df = pd.read_csv(target_file)
            print(df)
        except Exception as e:
            print(f"Error while updating the CSV file: {e}")
    else:
        print(f"Error: No 'experiment_go_*.csv' file found in {results_dir}")

if __name__ == "__main__":
    main()