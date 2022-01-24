import pandas as pd

def expand_csv(df, age, infected, vaccine, mask, room_num, save_path):
    df["age"] = age
    df["infected"] = infected
    df["vaccine"] = vaccine
    df["mask"] = mask
    df["room_num"] = room_num
    df.to_csv(save_path, header=False, index=False)
    return df

for i in range(1000):
    # path = "./trajectory_hash/data/tokyo/client-24-22-"+str(i)+".csv"
    path = "./trajectory_hash/data/25-25/client-25-25-"+str(i)+".csv"
    # save_path = "./trajectory_hash/data_expanded/tokyo/client-24-22-"+str(i)+".csv"
    save_path = "./trajectory_hash/data_expanded/25-25/client-25-25-"+str(i)+".csv"
    df = pd.read_csv(path, names=["hash_value",], dtype=str)
    expand_csv(df, 0,0,0,0,0, save_path=save_path)

path = "./trajectory_hash/data/25-25/server-1000-25-25.csv"
save_path = "./trajectory_hash/data_expanded/25-25/server-1000-25-25.csv"
df = pd.read_csv(path, names=["hash_value",], dtype=str)
expand_csv(df, 0,0,0,0,0, save_path=save_path)

path = "./trajectory_hash/data/25-25/server-5000-25-25.csv"
save_path = "./trajectory_hash/data_expanded/25-25/server-5000-25-25.csv"
df = pd.read_csv(path, names=["hash_value",], dtype=str)
expand_csv(df, 0,0,0,0,0, save_path=save_path)