# merge i file generati dalle varie iterazioni nel primo creato (ordine temporale)
import pathlib
import argparse
import pandas as pd

def get_session_files(directory, n):
    # Recupera gli ultimi n file experiment_rust_...csv creati.
    files = list(directory.glob("experiment_rust_*.csv"))
    if not files:
        return []
    # Ordina per tempo di modifica (dal più recente al più vecchio)
    files.sort(key=lambda x: x.stat().st_mtime, reverse=True)
    return files[:n]

def merge_session_results():
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', nargs='*', type=str)
    args = parser.parse_args()

    # Numero di iterazioni effettuate
    n = len(args.d) if args.d else 0
    
    if n <= 1:
        print("No merge necessary (0 or 1 dataset provided).")
        return

    # Percorso cartella risultati (adeguato alla tua struttura ..\..\results)
    p = pathlib.Path(__file__)
    output_dir = p.parents[2] / "results"
    
    # Recuperiamo gli n file più recenti (che corrispondono a questa sessione)
    # Li invertiamo per avere l'ordine cronologico [0] = il più vecchio, [-1] = l'ultimo
    session_files = get_session_files(output_dir, n)
    session_files.reverse() 

    if len(session_files) < 2:
        print(f"Only {len(session_files)} files found, unable to proceed with the merge.")
        return

    print(f"\n--- Merging {len(session_files)} files in progress ---")
    
    # Carichiamo il primo file della sessione
    main_df = pd.read_csv(session_files[0])
    
    # Appendiamo gli altri e li cancelliamo
    for extra_file in session_files[1:]:
        temp_df = pd.read_csv(extra_file)
        main_df = pd.concat([main_df, temp_df], ignore_index=True)
        extra_file.unlink() # Rimuove il file ridondante
        print(f"File {extra_file.name} merged and removed.")
    
    # Sovrascriviamo il primo file con il dataframe completo
    main_df.to_csv(session_files[0], index=False)
    print(f"Final result saved in: {session_files[0].name}")

if __name__ == "__main__":
    merge_session_results()