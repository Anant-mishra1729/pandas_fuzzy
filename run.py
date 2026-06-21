import pandas as pd
from pandas_fuzzy import fuzzy_filter

df = pd.DataFrame({"data": ["Anant", "John", "Johnas", "Rolles"]})
print(fuzzy_filter(df, "data", "Anan", score_cutoff=0.8))
