import pandas as pd

def expand_csv(df, age, infected, vaccine, mask, room_num, save_path):
    df["age"] = age
    df["infected"] = infected
    df["vaccine"] = vaccine
    df["mask"] = mask
    df["room_num"] = room_num
    df.to_csv(save_path, header=False, index=False)
    return df

for i in range(100):
    path = "./trajectory_hash/data/random/client-24-22-"+str(i)+".csv"
    save_path = "./trajectory_hash/data_expanded/random/client-24-22-"+str(i)+".csv"
    df = pd.read_csv(path, names=["hash_value",], dtype=str)
    expand_csv(df, 0,0,0,0,0, save_path=save_path)

path = "./trajectory_hash/data/random/server-1000-24-22.csv"
save_path = "./trajectory_hash/data_expanded/random/server-1000-24-22.csv"
df = pd.read_csv(path, names=["hash_value",], dtype=str)
expand_csv(df, 0,0,0,0,0, save_path=save_path)

# PCT/tools/trajectory_hash/data/tokyo/client-24-22-1.csv
# PCT/tools/trajectory_hash/data_expanded
# PCT_original/tools/trajectory_hash/data/tokyo/server-14000-24-22.csv