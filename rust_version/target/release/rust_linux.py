import pathlib
import argparse
import subprocess
import pandas as pd
from codecarbon import OfflineEmissionsTracker

def get_latest_experiment_file(directory):
    """Trova il file experiment_rust_...csv più recente in assoluto."""
    files = list(directory.glob("experiment_rust_*.csv"))
    if not files:
        return None
    return max(files, key=lambda x: x.stat().st_mtime)

def run_experiment():
    parser = argparse.ArgumentParser(description="Runner per programma Rust con monitoraggio CodeCarbon")
    parser.add_argument('-d', nargs='*', type=str, help='Vettore numerico')
    parser.add_argument('-t', nargs='*', type=str, help='Vettore di stringhe')
    
    args = parser.parse_args()
    d_vector = args.d if args.d is not None else []
    t_vector = args.t if args.t is not None else []

    if len(t_vector) > 0 and len(d_vector) != len(t_vector):
        print("Errore: I vettori -d e -t devono avere la stessa lunghezza.")
        return

    p = pathlib.Path(__file__)
    output_dir = p.parents[2] / "results"
    output_dir.mkdir(parents=True, exist_ok=True)
    csv_carbon = output_dir / "emissions.csv"

    iterations = max(len(d_vector), 1)
    # Lista per tenere traccia dei file generati in questa specifica sessione
    session_files = []

    for i in range(iterations):
        print(f"\n--- Inizio Iterazione {i+1}/{iterations} ---")
        
        cmd_list = [str(p.parent / "rust_version.exe")]
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
            print(f"Errore durante l'esecuzione di Rust: {e}")
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
                print(f"Dato ({ultimo_consumo} kWh) inserito in: {target_file.name}")
            
            csv_carbon.unlink()

    # --- FASE DI FUSIONE (MERGE) ---
    if len(session_files) > 1:
        print(f"\n--- Fusione di {len(session_files)} file in corso ---")
        
        # Carichiamo il primo file (il più "vecchio" della sessione attuale)
        main_df = pd.read_csv(session_files[0])
        
        # Appendiamo gli altri
        for extra_file in session_files[1:]:
            temp_df = pd.read_csv(extra_file)
            main_df = pd.concat([main_df, temp_df], ignore_index=True)
            # Eliminiamo il file ridondante dopo la fusione
            extra_file.unlink()
            print(f"File {extra_file.name} fuso e rimosso.")
        
        # Salviamo tutto nel primo file della sessione
        main_df.to_csv(session_files[0], index=False)
        print(f"Risultato finale salvato in: {session_files[0].name}")
    else:
        print("\nNessuna fusione necessaria (singolo file o nessun file generato).")

if __name__ == "__main__":
    run_experiment()