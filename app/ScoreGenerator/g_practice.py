import pandas as pd
#from pulp import LpProblem, LpVariable, LpMinimize, lpSum, LpStatus, value
import random

# 输入参数
excel_file = './sample.xlsx'
col_name = 'score'  # 需要读取的总分列名
n = 2  # 项数
coeffs = [0.6, 0.4]  # 报告成绩和答辩成绩系数
n1 = 3 
coeffs1 = [0.4, 0.3, 0.3]  # 报告成绩组成比例 
n2 = 2 
coeffs2 = [0.5, 0.5]  # 答辩成绩组成比例 
# 读取Excel
df = pd.read_excel(excel_file)
#cols = len(df.columns)
#print(cols)
numbers = df[col_name].dropna().tolist()
result_cols = ['g1', 'g11', 'g12', 'g13', 'g2', 'g21', 'g22']

for col in result_cols:
    df[col] = None
for idx, num in enumerate(numbers):
    success = False
    g_num = [0 for i in range(n + n1 + n2)]
    g_num[0] = int(num)
    while success == False:
        
        #step1, decompose the final score
        
        g_num[4] = (g_num[0]//10)*10 + random.uniform(0, 10 - g_num[0]//10)

        #step2, decompose g[0], the report score
        s1 = False
        while not s1:
            g_num[1] = (g_num[0] + random.uniform(-10, 98 - g_num[0])) * coeffs1[0]
            g_num[2] = (g_num[0] + random.uniform(-10, 98 - g_num[0])) * coeffs1[1]
            g_num[3] = (g_num[0] - g_num[1] - g_num[2]) / coeffs1[2]
            if g_num[3] > 98 or g_num[3] < 50:
                continue
            s1 = True
        g_num[3] = g_num[3] * coeffs1[2]

        #step3, decompose g[1], the performance score
        s2 = False
        while not s2:
            g_num[5] = (g_num[4] + random.uniform(-10, 98 - g_num[0])) * coeffs2[0]
            g_num[6] = (g_num[4] - g_num[5]) / coeffs2[1]
            if g_num[6] > 98 or g_num[6] < 50:
                continue
            s2 = True
        g_num[6] = g_num[6] * coeffs2[1]


        for i in range(n + n1 + n2):
            df.loc[idx, result_cols[i]] = int(g_num[i])
        success = True

df.to_excel("r.xlsx", index=False)