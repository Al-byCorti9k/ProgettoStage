# Script che serve per calcolare i consumi energetici di un programma rust con l'interfaccia RAPL tramite CodeCarbon
import pathlib
import argparse
import subprocess
import pandas as pd
from codecarbon import OfflineEmissionsTracker

def get_latest_experiment_file(directory):
    # Trova il file experiment_rust_...csv più recente in assoluto.
    files = list(directory.glob("experiment_rust_*.csv"))
    if not files:
        return None
    return max(files, key=lambda x: x.stat().st_mtime)

def run_experiment():
    parser = argparse.ArgumentParser(description="Runner for Rust program with CodeCarbon monitoring")
    parser.add_argument('-d', nargs='*', type=str, help='Datasets  ID')
    parser.add_argument('-t', nargs='*', type=str, help='Target column names')
    
    args = parser.parse_args()
    d_vector = args.d if args.d is not None else []
    t_vector = args.t if args.t is not None else []

    if len(t_vector) > 0 and len(d_vector) != len(t_vector):
        print("Error: vector -d and -t must have the same lenght.")
        return

    p = pathlib.Path(__file__)
    output_dir = p.parents[2] / "results"
    output_dir.mkdir(parents=True, exist_ok=True)
    csv_carbon = output_dir / "emissions.csv"

    iterations = max(len(d_vector), 1)
    # Lista per tenere traccia dei file generati in questa specifica sessione
    session_files = []

    for i in range(iterations):
        print(f"\n--- Starting Iteration {i+1}/{iterations} ---")
        
        cmd_list = [str(p.parent / "rust_version")]
        if d_vector:
            cmd_list.extend(["-d", d_vector[i]])
        if t_vector:
            cmd_list.extend(["-t", t_vector[i]])

        tracker = OfflineEmissionsTracker(
            country_iso_code="ITA",
            output_dir=str(output_dir),
            measure_power_secs=1,
            tracking_mode="process",
            on_csv_write="append"
        )
        
        tracker.start()
        try:
            subprocess.run(cmd_list, check=True)
        except Exception as e:
            print(f"Error during Rust execution: {e}")
        finally:
            tracker.stop()

        # Elaborazione del consumo energetico
        if csv_carbon.exists():
            df_carbon = pd.read_csv(csv_carbon)
            ultimo_consumo = df_carbon.iloc[-1]['energy_consumed']
            
            # Troviamo il file appena creato da Rust
            target_file = get_latest_experiment_file(output_dir)
            
            if target_file and target_file not in session_files:
                session_files.append(target_file)
                df_exp = pd.read_csv(target_file)
                
                # Inseriamo il dato nella colonna specifica (su tutte le righe del file se Rust ne produce più di una, 
                # o aggiungiamo la colonna se non esiste)
                df_exp["energy consumption (kWh)"] = ultimo_consumo
                
                df_exp.to_csv(target_file, index=False)
                print(f"\n Data ({ultimo_consumo} kWh) entered in: {target_file.name}")
            
            csv_carbon.unlink()

    # --- FASE DI FUSIONE (MERGE) ---
    if len(session_files) > 1:
        print(f"\n--- Merging {len(session_files)} files in progress ---")
        
        # Carichiamo il primo file (il più "vecchio" della sessione attuale)
        main_df = pd.read_csv(session_files[0])
        
        # Appendiamo gli altri
        for extra_file in session_files[1:]:
            temp_df = pd.read_csv(extra_file)
            main_df = pd.concat([main_df, temp_df], ignore_index=True)
            # Eliminiamo il file ridondante dopo la fusione
            extra_file.unlink()
            print(f"File {extra_file.name} merged and removed.")
        
        # Salviamo tutto nel primo file della sessione
        main_df.to_csv(session_files[0], index=False)
        print(f"Final results saved in: {session_files[0].name}")
    else:
        print("\nNo merge necessary (single file or no files generated).")

if __name__ == "__main__":
    run_experiment()
