#实训成绩分解
import pandas as pd
import numpy as np
import random

excel_file = './sample.xlsx'
col_name = 'score'
output_file = './result.xlsx'

n = 2
coeffs = [0.6, 0.4]

n_x = [3, 2]
coeffs_x = [[0.4, 0.3, 0.3], [0.5, 0.5]]

def decompose_score(score, n, coeffs):
    if score == 0:
        return [0.0] * n

    low = max(0, score - 10)
    high = min(97, score + 10)

    s = [float(high)] * n
    offset = int(high - score)
    nums = list(range(n))

    for _ in range(offset):
        if not nums:
            break
        idx = random.choice(nums)
        s[idx] -= 1 / coeffs[idx]
        if s[idx] < low:
            nums.remove(idx)

    return s

# 读取Excel文件
df = pd.read_excel(excel_file)

if col_name not in df.columns:
    raise ValueError(f"列名 '{col_name}' 不存在于Excel文件中")

# 两级分解
level1_cols = []
level2_cols = []

for i in range(n):
    level1_cols.append(f'g_{i+1}')
    for j in range(n_x[i]):
        level2_cols.append(f'g_{i+1}_{j+1}')

all_new_cols = level1_cols + level2_cols

def two_level_decompose(score):
    g = decompose_score(score, n, coeffs)
    row = {}
    for i in range(n):
        row[f'g_{i+1}'] = g[i]
        sub = decompose_score(g[i], n_x[i], coeffs_x[i])
        for j in range(n_x[i]):
            row[f'g_{i+1}_{j+1}'] = sub[j]
    return row

results = df[col_name].apply(two_level_decompose)
decomposed_df = pd.DataFrame(results.tolist(), index=df.index)

result_df = pd.concat([df, decomposed_df], axis=1)

try:
    result_df.to_excel(output_file, index=False)
    print(f"分解完成，结果已保存到 {output_file}")
except PermissionError:
    print("result.xlsx 被占用，跳过保存，仅输出到控制台")

print(f"共处理 {len(df)} 条记录")
print(f"\n一级系数: coeffs = {coeffs}")
print(f"二级系数: coeffs_x = {coeffs_x}")
print(f"\n前10条记录的分解结果:")
print(result_df[all_new_cols].head(10).to_string())
