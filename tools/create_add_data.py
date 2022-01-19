import csv

def create_hashed_data(type: str, age: int, infected: int, vaccine: int, mask: int):
    if type == "server":
        l = [["01a451cc30592513", age, infected, vaccine, mask],
             ["01a451cc30d82503", age, infected, vaccine, mask],
             ["01a451cc344368b3", age, infected, vaccine, mask],
             ["01a451cc34c20db3", age, infected, vaccine, mask],
             ["01a451cd100948b3", age, infected, vaccine, mask],
             ["01a451cc16902533", age, infected, vaccine, mask],
             ["01a451cc16902886", age, infected, vaccine, mask],
             ["01a451cc16900d92", age, infected, vaccine, mask],
             ["01a451cc16900d04", age, infected, vaccine, mask],
             ["01a451cc16904010", age, infected, vaccine, mask]]
        with open('./../data/test/server.csv', 'w') as f:
            writer = csv.writer(f)
            for i in range(144):
                writer.writerows(l)
    elif type == "client_0":
        with open('./../data/test/client-24-22-0-test.csv', 'w') as f:
            writer = csv.writer(f)
            for i in range(1439):
                writer.writerow(["0000000000000000", age, infected, vaccine, mask])
    elif type == "client_1":
        with open('./../data/test/client-24-22-1-test.csv', 'w') as f:
            writer = csv.writer(f)
            writer.writerow(["01a451cc30592513", age, infected, vaccine, mask])
            for i in range(1438):
                writer.writerow(["1111111111111111", age, infected, vaccine, mask])
    else:
        raise Exception('Error! Please select a correct type{server,client_0,client_1}.')

create_hashed_data("server", 1, 0, 0, 0)
create_hashed_data("client_0", 1, 0, 0, 0)
create_hashed_data("client_1", 1, 0, 0, 0)