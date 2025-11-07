import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import pathlib
from sklearn import linear_model
import array
#ottengo il percorso del file Main.py
p = pathlib.Path(__file__)

#ottengo il percorso del file .csv
p_csv = pathlib.PurePath(p).parents[2].joinpath("data", "100_7717_peerj_5665_dataYM2018_neuroblastoma.csv")

#importo il dataset in un oggetto dataframe
df = pd.read_csv(p_csv)
print(df.tail(5))

#ottengo tutte le colonne
df_columns = df.columns[:-1].tolist()


reg = linear_model.LinearRegression()
# le doppie [ servono a creare un dataframe a partire da un altro
reg.fit(df[df_columns], df.outcome)

#test
print(reg.predict(df.iloc[[-1], :-1]))
print(reg.predict(df.iloc[[-4], :-1]))
