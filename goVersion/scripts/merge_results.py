# merge i file generati dalle varie iterazioni nel primo creato (ordine temporale)
import pathlib
import argparse
import pandas as pd

def get_session_files(directory, n):
    """Recupera gli ultimi n file experiment_go_...csv creati."""
    files = list(directory.glob("experiment_go_*.csv"))  # <-- cambiato da rust a go
    if not files:
        return []
    # Ordina per tempo di modifica (dal più recente al più vecchio)
    files.sort(key=lambda x: x.stat().st_mtime, reverse=True)
    return files[:n]

def merge_session_results():
    parser = argparse.ArgumentParser()
    parser.add_argument('-d', nargs='*', type=str)
    parser.add_argument('-t', nargs='*', type=str)
    args = parser.parse_args()

    n = len(args.d) if args.d else 0

    if n <= 1:
        print("Nessuna fusione necessaria (0 o 1 dataset fornito).")
        return

    # --- GESTIONE PERCORSI CORRETTA ---
    p = pathlib.Path(__file__)
    output_dir = p.parent.parent / "results"  # radice/results

    # Recuperiamo gli n file più recenti (che corrispondono a questa sessione)
    session_files = get_session_files(output_dir, n)
    # Invertiamo per avere ordine cronologico: [0] = più vecchio, [-1] = ultimo
    session_files.reverse()

    if len(session_files) < 2:
        print(f"Trovati solo {len(session_files)} file, impossibile procedere alla fusione.")
        return

    print(f"\n--- Fusione di {len(session_files)} file in corso ---")

    # Carichiamo il primo file della sessione (il più vecchio)
    main_df = pd.read_csv(session_files[0])

    # Appendiamo gli altri e li cancelliamo
    for extra_file in session_files[1:]:
        temp_df = pd.read_csv(extra_file)
        main_df = pd.concat([main_df, temp_df], ignore_index=True)
        extra_file.unlink()
        print(f"File {extra_file.name} fuso e rimosso.")

    # Sovrascriviamo il primo file con il dataframe completo
    main_df.to_csv(session_files[0], index=False)
    print(f"Risultato finale salvato in: {session_files[0].name}")

if __name__ == "__main__":
    merge_session_results()