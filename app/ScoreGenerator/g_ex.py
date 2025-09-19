import pandas as pd
#from pulp import LpProblem, LpVariable, LpMinimize, lpSum, LpStatus, value
import random

excel_file = './ex.xlsx'
m = 6  # the number of experiments
n = 3  # the divided number of every item
coeffs = [0.4, 0.4, 0.2]  # proportation

def decompose(num, coff, n):
    val = [0 for i in range(n)]

    success = False
    while not success:
        sum = 0
        for i in range(n - 1):
            inc = random.uniform(-10, 98 - num)
            val[i] = num + inc
            sum += val[i] * coff[i]
        
        val[n-1] = (num - sum)/coff[n-1]

        if val[n-1] > 98 or val[n-1] < 60:
            continue
        success = True

    for i in range(n):
        val[i] = int(val[i] * coff[i])
    return val


# 输入参数

# 读取Excel
df = pd.read_excel(excel_file)
#cols = len(df.columns)
#print(cols)

result_cols = [f's{i}{j}' for i in range(m) for j in range(n)]

for col in result_cols:
    df[col] = None

for i in range(m):
    col_name = f'ex{i+1}'
    numbers = df[col_name].dropna().tolist()
    for idx, num in enumerate(numbers):
        val = decompose(num, coeffs, n)
        for j in range(n):
            df.loc[idx, result_cols[i*n + j]] = val[j]
    
df.to_excel("r.xlsx", index=False)

