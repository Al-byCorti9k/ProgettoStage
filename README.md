Program for computing LOOCV, MCC and stimated energy consumption with external
tools (like CodeCarbon and Intel VTune Profiler) of a linear regression model.

- [About the project](#about-the-project)
- [Requirements](#requirements)
  - [Intel VTune Profiler Installation's Guide](#intel-vtune-profiler-installations-guide)
  - [CodeCarbon on Linux](#codecarbon-on-linux)
- [Python's version](#pythons-version)
  - [Installation](#installation)
  - [Quickstart](#quickstart)
- [License](#license)

# About the project
This project represents my thesis work for the Bachelor’s Degree in Computer Science at the University of [Milano-Bicocca](https://www.unimib.it/).

At the moment is an ongoing project that aim to estimate the energy consumption of a LOOCV's linear regression model in three different programming languages:
- Python (complete)
- Rust (ongoing)
- GO (ongoing)

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
/sys/class/powercap/intel-rapl/subsystem
```
# Python's version
You'll need to run python 3.14. I suggest you to install all the packets in a separated enviroment. My guide will illustrate how to create a miniconda3 enviroment to run properly this program.
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
You'll get all the results on the terminal and on a specific .CSV file at the default fold "results"
# License

This project integrates with Intel® VTune™ Profiler through its command-line interface (CLI).
Intel® VTune™ Profiler is not distributed with this project. 
Users must install VTune separately and ensure compliance with Intel’s End User License Agreement.

Performance analysis results generated using Intel® VTune™ Profiler.
Intel® VTune™ Profiler is the property of Intel Corporation.

This project uses the CodeCarbon library as a dependency.
CodeCarbon is not distributed with this project and must be installed separately.
Its use is subject to the terms of its original open-source license