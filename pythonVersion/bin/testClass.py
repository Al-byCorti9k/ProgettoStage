import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import pathlib
from sklearn.preprocessing import StandardScaler, OneHotEncoder
from sklearn.compose import ColumnTransformer, make_column_selector as selector
from sklearn import linear_model, model_selection 
from sklearn.metrics import matthews_corrcoef
from sklearn.pipeline import make_pipeline


# Versione Python del classificatore binario con regressione lineare e LeaveOneOut cross validation
# per progetto di stage e tesi presso l'università Milano Bicocca

# ottengo il percorso del file Main.py
p = pathlib.Path(__file__)

# ottengo il percorso del file .csv
name_csv = "Takashi2019_diabetes_type1_dataset_preprocessed.csv"
p_csv = pathlib.PurePath(p).parents[2].joinpath("data", name_csv)
df = pd.read_csv(p_csv)

print(df.dtypes)