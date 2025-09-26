import pandas as pd

#assume the two excel file have the same header with two-layer index
msg = ['一学期必修课程或专业选修课考试不及格',
       '入校以来平均学分绩点在 2.0 以下',
       '一学期不及格课程（不含公共选修课）达 6 学分（含）,以上 9 学分（不含）以下者',
       '入学以来累计不及格课程 6 学分（含）以上 12 学分（不含）以下',
       '一学期不及格课程（不含公共选修课）达 9 学分（含）以上',
       '入学以来累计不及格课程达 12 学分及以上']
headers = [3, 4] # the row number of the table header
col0 = ['通识必修', '专业必修', '专业选修', '学号', '姓名'] #general compulsory, special compulsory, special option, sname, sno
col1 = ['修读学分', '获得学分'] #required, obtained 
cols_useful = [('学号', 'Unnamed: 1_level_1'), ('姓名','Unnamed: 2_level_1'), ('通识必修', '修读学分'), ('通识必修', '获得学分'),('专业必修', '修读学分'), ('专业必修', '获得学分'), ('专业选修', '修读学分'), ('专业选修', '获得学分'), ('综合', '平均学分绩点')]
result_cols = ['sno', 'sname', 'diff', 'avg']

def read_and_extract_excel(file_name):
    # step1 read the excel file
    df = pd.read_excel(file_name, header = headers)
    print(df.columns)
    rows = len(df)
    print('rows=', rows)
    # step2 extract the column num of useful information
    
    # step3 row by row compute the difference of obtained and required 
    dataframe = pd.DataFrame(columns = result_cols)
    dataframe['sno'] = df[('学号', 'Unnamed: 1_level_1')]
    dataframe['sname'] = df[('姓名','Unnamed: 2_level_1')]
    dataframe['diff'] = (df[('通识必修', '修读学分')] - df[('通识必修', '获得学分')]) \
        + (df[('专业必修', '修读学分')] - df[('专业必修', '获得学分')]) \
        + (df[('专业选修', '修读学分')] - df[('专业选修', '获得学分')]) 
    dataframe['avg'] = df[('综合', '平均学分绩点')]
    return dataframe

# cols=['sno', 'sname']
# current is the first datframe and histroy is the second dataframe 
def merge_and_warn(df1, df2):
    print(df1.columns)
    print(df2.columns)
    df = pd.merge(df1, df2, on = ['sno', 'sname'], how = 'outer', suffixes = ('_cur', '_his'))
    print(df.columns)
    result_cols = ['sno', 'sname', 'diff_cur', 'diff_his', 'avg_cur', 'avg_his', 'warn', 'proof']
    r = pd.DataFrame(columns=result_cols)
    
    rows = len(df)
    index = 0
    for i in range(rows):
        warning_msg = []
        warning_signal = None

        j = df.columns.get_loc('diff_cur') 
        if df.iloc[i, j] > 0:
            warning_msg.append(msg[0])
            warning_signal = '黄色'
        j = df.columns.get_loc('avg_his')
        if df.iloc[i, j] < 2.0:
            warning_msg.append(msg[1])
            warning_signal = '橙色'
        j = df.columns.get_loc('diff_cur')
        if df.iloc[i, j] >= 6 and df.iloc[i, j] < 9:
            warning_msg.append(msg[2])
            warning_signal = '橙色'
        j = df.columns.get_loc('diff_his')
        if df.iloc[i, j] >= 6 and df.iloc[i, j] < 12:
            warning_msg.append(msg[3])
            warning_signal = '橙色'
        j = df.columns.get_loc('diff_cur')
        if df.iloc[i, j] >= 9:
            warning_msg.append(msg[4])
            warning_signal = '红色'
        j = df.columns.get_loc('diff_his')
        if df.iloc[i, j] >= 12:
            warning_msg.append(msg[5])
            warning_signal = '红色'

        if len(warning_msg) > 0:
            j = df.columns.get_loc('sno')
            sno = df.iloc[i, j]
            j = df.columns.get_loc('sname')
            sname = df.iloc[i, j]
            j = df.columns.get_loc('diff_cur')
            diff_cur = df.iloc[i, j]
            j = df.columns.get_loc('diff_his')
            diff_his = df.iloc[i, j]
            j = df.columns.get_loc('avg_cur')
            avg_cur = df.iloc[i, j]
            j = df.columns.get_loc('avg_his')
            avg_his = df.iloc[i, j]
            j = r.columns.get_loc('warn')
            warn = warning_signal
            proof = '\n'.join(warning_msg)
            r.loc[index] = [sno, sname, diff_cur, diff_his, avg_cur, avg_his, warn, proof]
            index += 1
    return r

if __name__ == "__main__":
    df1 = read_and_extract_excel('current.xls')
    df2 = read_and_extract_excel('history.xls')
    df = merge_and_warn(df1, df2)
    df.to_excel("r.xlsx")