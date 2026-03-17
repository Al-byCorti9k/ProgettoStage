Program for computing LOOCV, MCC and stimated energy consumption with external
tools (like CodeCarbon and Intel VTune Profiler) of a linear regression model.

- [About the project](#about-the-project)
- [Requirements](#requirements)
  - [Intel VTune Profiler Installation's Guide](#intel-vtune-profiler-installations-guide)
  - [CodeCarbon on Linux](#codecarbon-on-linux)
- [Python's version](#pythons-version)
  - [Installation](#installation)
  - [Quickstart](#quickstart)
- [Rust's version](#rusts-version)
    - [Installation](#installation-1)
    - [Quickstart](#quickstart-1)
- [Go's version](#gos-version)
    - [Installation](#installation-2)
    - [Quickstart](#quickstart-2)
- [License](#license)

# About the project
This project represents my thesis work for the Bachelor’s Degree in Computer Science at the University of [Milano-Bicocca](https://www.unimib.it/).

This project aims to estimate the energy consumption of a LOOCV's linear regression model in three different programming languages:
- Python (complete)
- Rust (complete)
- GO (complete)

The model is trained and verified on five medical datasets, available at the "data" directory at the same level ad the "bin" directory. 
At the end, you'll be able to make a compararision analysis 

# Requirements
The project now rely on [CodeCarbon](https://github.com/mlco2/codecarbon) (v3.2.0) and [Intel VTune Profiler](https://www.intel.com/content/www/us/en/developer/tools/oneapi/vtune-profiler.html)(v2025.4) for the computation of energy's consumption.

The supported operating Systems are Windows and Linux. I made my tests on a machine with this characteristics:

- Processor:	Intel(R) Core(TM) i7-1065G7 CPU @ 1.30GHz (1.50 GHz)
- RAM:	16,0 GB 

on Windows 11 and Linux Mint-22.2-cinnamon-64bit. Intel VTune Profiler works only on Intel CPU, but CodeCarbon comes with support for all CPU with RAPL interface, so both Intel and AMD processor that respects these conditions are allowed. 
To be fair, My code Itself has no particolar system's requirements; all restriction derive from the module installed for the energy computation. For major detail of how it works, I suggest to visit [this](https://mlco2.github.io/codecarbon/methodology.html)
and [this](https://www.intel.com/content/www/us/en/docs/vtune-profiler/get-started-guide/2025-4/overview.html)

Then, you'll need to create a folder named "results" at the same level of the "bin" folder of each program's version.

## Intel VTune Profiler Installation's Guide
You need to follow the official guide from the [Intel's website](https://www.intel.com/content/www/us/en/docs/vtune-profiler/installation-guide/2025-0/windows.html).
Since my program just use two Intel VTune Profiler command CLI version to get the datas, you should install only as a [Standalone Application](https://www.intel.com/content/www/us/en/docs/vtune-profiler/installation-guide/2025-0/windows.html#:~:text=Install%20VTune%20Profiler%20as%20a%20Standalone%20Application), then follow the after installation instruction to verify your system and your drivers. 

## CodeCarbon on Linux
you need to install the RAPL module to let codeCarbon use its interface:
```bash
modprobe intel_rapl_common
```
and you need to give RAPL files read permission:
```bash
sudo chmod -R a+r /sys/class/powercap/*
```
# Python's version
You'll need to run python 3.14. I suggest you to install all the packets in a separated enviroment. My guide will illustrate how to create a miniconda3 enviroment to run properly this program.

**Note**: you have to run the program in a conda enviroment if you use Intel VTune Profiler. 
## Installation
 - install minicoda following this [guide](https://www.anaconda.com/docs/getting-started/miniconda/install#quickstart-install-instructions)
 - Then create a specific enviroment with a command like this:
```bash
conda create -n Enviroment_name python=3.14
```
then you'll need these packages:
```bash
conda activate your_enviroment
```
```python
pip install scikit-learn pandas codecarbon ipython
```
All version are fine if they work with python 3.14.

Then you are ready to go! 

## Quickstart
to get info about this program's flags:
```bash
python Main.py -h
```
on Windows this will launch the default mode, with a call to dataset "Neuroblastoma" and energy consumptions calculated with Intel VTune Profiler (on Linux the default is CodeCarbon)
```bash
python Main.py
```
you can choose a specific dataset to train the model with in this way:
```bash
python Main.py -i 1
```
you can consult the available dataset in this way:
```bash
python Main.py -al
```
You'll get all the results on the terminal and on a specific .CSV file at the default fold "results" at `.\pythoVersion\` directory.


# Rust's version

The Rust implementation performs the same LOOCV linear regression analysis on the medical datasets, with energy measurement support via Intel VTune Profiler (Windows) or CodeCarbon (Linux).

### Installation

1. Navigate to the `rust_version` directory:
   ```bash
   cd rust_version
   ```
   Ensure the `Cargo.toml` manifest file is present.

2. Build the release binary:
   ```bash
   cargo build --release
   ```
   This will generate an executable named `rust_version` (Linux/macOS) or `rust_version.exe` (Windows) inside the `.\target\release` directory.

3. **Energy measurement prerequisites**  
   - **On Windows**: You need **PowerShell with administrator privileges** to run the `vtuneRust.ps1` script. This script invokes Intel VTune Profiler (which must be installed separately, see [Intel VTune Profiler Installation's Guide](#intel-vtune-profiler-installations-guide)) and passes the appropriate parameters to the Rust executable.  
   - **On Linux**: Use the Python script `rust_linux.py`. It relies on [CodeCarbon](https://github.com/mlco2/codecarbon) to track energy consumption.
   -  **Both scripts** require Python with the `pandas` library installed. You can install pandas via `pip install pandas` if not already present. The script will launch the Rust binary and monitor its energy usage.

### Quickstart

- **Run with energy measurement**  
  - Windows (PowerShell as Administrator, from the `.\target\release\` directory):
    ```powershell
    ./vtuneRust.ps1 -d 3 -t "your_target_name"
    ```
    The `-d` flag selects the dataset, and `-t` is an optional label for the output file.
  - Linux (from the `.\target\release\` directory):
    ```bash
    python rust_linux.py -d 3 -t "your_target_name"
    ```

- **Run without energy measurement** (direct execution from the `.\target\release\` directory)  
  - Display help:
    ```bash
    ./rust_version.exe -h # windows
    ./rust_version -h # linux
    ```
  - Show the dataset index mapping:
    ```bash
    ./rust_version.exe -l # windows
    ./rust_version -h # linux
    ```
  - Normal execution with dataset index 3 (automatically select the last column as target):
    ```bash
    ./rust_version.exe -d 3 # windows
    ./rust_version -h # linux
    ```

All results (metrics and energy data) will be saved in the `results` folder as CSV files `.\rust_version\` directory.

# Go's version

The Go implementation replicates the LOOCV linear regression analysis on the same medical datasets, with energy measurement support via Intel VTune Profiler (Windows) or CodeCarbon (Linux). The structure and usage closely follow the Rust version.

### Installation

1. Navigate to the `goVersion\src` directory:
   ```bash
   cd goVersion\src
   ```

2. Install the required Go dependencies:
   ```bash
   go mod tidy
   ```
   This command downloads and prepares all modules listed in `go.mod`.

3. Build the executable:
   ```bash
   go build -o goVersion.exe
   ```
   - **On Windows**: this produces `goVersion.exe` in the current directory (`goVersion\src`).
   - **On Linux/macOS**: you may want to omit the `.exe` extension, e.g. `go build -o goVersion`. However, the measurement scripts expect the executable to be named **exactly `goVersion.exe`** (Windows) or `goVersion` (Linux).

4. **Energy measurement prerequisites**  
   - **On Windows**: After building the executable, you can measure energy consumption using the Python script `vtuneGo.py` located in the `goVersion\scripts` directory. This script invokes Intel VTune Profiler (see [Intel VTune Profiler Installation's Guide](#intel-vtune-profiler-installations-guide)) and passes the necessary parameters to `goVersion.exe`.  
   - **On Linux**: Use the Python script `go_linux.py` in the same `scripts` folder. It relies on [CodeCarbon](https://github.com/mlco2/codecarbon) to track energy usage.  
   - **Both scripts** require Python with the `pandas` library installed (`pip install pandas` if needed).

### Quickstart

- **Run with energy measurement**  
  - Windows (PowerShell or Command Prompt, with administrator privileges for VTune):
    ```bash
    cd goVersion\scripts
    python vtuneGo.py -d 3 -t "your_target_name"
    ```
  - Linux:
    ```bash
    cd goVersion/scripts
    python go_linux.py -d 3 -t "your_target_name"
    ```
  The `-d` flag selects the dataset index, and `-t` is an optional label for the output file.

- **Run without energy measurement** (direct execution)  
  - Display help:
    ```bash
    ./goVersion.exe -h        # on Windows
    ./goVersion -h            # on Linux (if built without .exe)
    ```
  - Show the dataset index mapping::
    ```bash
    ./goVersion.exe -v        # on Windows
    ./goVersion -v            # on Linux (if built without .exe)
    ```
  - Normal execution with dataset index 3:
    ```bash
    ./goVersion.exe -d 3
    ```

All results (performance metrics and energy data) are saved as CSV files in the `goVersion/results` directory.


# License

This project integrates with Intel® VTune™ Profiler through its command-line interface (CLI).
Intel® VTune™ Profiler is not distributed with this project. 
Users must install VTune separately and ensure compliance with Intel’s End User License Agreement.

Performance analysis results generated using Intel® VTune™ Profiler.
Intel® VTune™ Profiler is the property of Intel Corporation.

This project uses the CodeCarbon library as a dependency.
CodeCarbon is not distributed with this project and must be installed separately.
Its use is subject to the terms of its original open-source license