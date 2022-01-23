import pandas as pd

def expand_csv(df, age, infected, vaccine, mask, room_num, save_path):
    df["age"] = age
    df["infected"] = infected
    df["vaccine"] = vaccine
    df["mask"] = mask
    df["room_num"] = room_num
    df.to_csv(save_path, header=False, index=False)
    return df

for i in range(2):
    path = "./../data/sample/client-24-22-"+str(i)+"-sample.csv"
    save_path = "./../data/temp/client-24-22-"+str(i)+"-sample.csv"
    df = pd.read_csv(path, names=["hash_value",], dtype=str)
    expand_csv(df, 0,0,0,0,0, save_path=save_path)

path = "./../data/sample/server.csv"
save_path = "./../data/temp/server.csv"
df = pd.read_csv(path, names=["hash_value",], dtype=str)
expand_csv(df, 0,0,0,0,0, save_path=save_path)