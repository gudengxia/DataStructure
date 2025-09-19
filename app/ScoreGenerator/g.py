import pandas as pd
#from pulp import LpProblem, LpVariable, LpMinimize, lpSum, LpStatus, value
import random

# 输入参数
excel_file = './sample.xlsx'
col_name = 'score'  # 需要读取的列名
n = 3  # 项数
coeffs = [0.5, 0.3, 0.2]  # n个小数

# 读取Excel
df = pd.read_excel(excel_file)
#cols = len(df.columns)
#print(cols)
numbers = df[col_name].dropna().tolist()
result_cols = [f'分解{i+1}' for i in range(n)]

for col in result_cols:
    df[col] = None
for idx, num in enumerate(numbers):
    success = False
    while success == False:
        sum = 0
        g_num = [0 for i in range(n)]
        for i in range(0, n-1):
            inc = random.uniform(-10, 98 - num)
            g_num[i] = num + inc
            sum += g_num[i] * coeffs[i]
        
        the_last_num = (num  - sum) / coeffs[n-1]
        if the_last_num <= 98 and the_last_num >= 50:
            success = True
            g_num[n-1] = the_last_num
            for i in range(n):
                #print(idx, i)
                df.loc[idx, result_cols[i]] = g_num[i]

df.to_excel("r.xlsx", index=False)