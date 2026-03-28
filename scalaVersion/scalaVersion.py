#!/usr/bin/env python3
import argparse
import subprocess
import pathlib
import sys

def installdependencies():

	libraries = ["codecarbon", "pandas"]

	for lib in libraries:
		try:
			__import__(lib)
			print(f"{lib} is already installed.")
		except ImportError:
			print(f"{lib} not found. Installing...")
			subprocess.check_call([sys.executable, "-m", "pip", "install", lib])


def main():
    parser = argparse.ArgumentParser(description="Scala Energy's consumption with codecarbon")
    parser.add_argument("filename", help="csv file name")
    args = parser.parse_args()

    installdependencies()

    import pandas as pd
    from codecarbon import OfflineEmissionsTracker

    # Directory per i risultati 
    output_dir = pathlib.Path.cwd()

    tracker = OfflineEmissionsTracker(
        country_iso_code="ITA",
        output_dir=str(output_dir),
        measure_power_secs=1,
        tracking_mode="process",
        on_csv_write="append",  
        log_level="critical")

    # Comando da eseguire
    cmd = ["java", "-jar", "scalaVersion.jar", args.filename]

    tracker.start()
    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"scala error: {e}", file=sys.stderr)
    finally:
        tracker.stop()

    # Leggi il file delle emissioni per ottenere il consumo energetico
    csv_carbon = output_dir / "emissions.csv"
    if csv_carbon.exists():
        df_carbon = pd.read_csv(csv_carbon)
        # Prendi l'ultimo valore di energy_consumed
        ultimo_consumo = df_carbon.iloc[-1]['energy_consumed']
        print(f"Energy consumption: {ultimo_consumo} kWh")
       
        csv_carbon.unlink()
    else:
        print("No data.", file=sys.stderr)

if __name__ == "__main__":
    main()