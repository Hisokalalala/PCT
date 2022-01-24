import csv

def create_hashed_data(type: str, age: int, infected: int, vaccine: int, mask: int, room: int, cli_num: str):
    if type == "server":
        l = [["01a451cc305925", age, infected, vaccine, mask, room],
             ["01a451cc30d825", age, infected, vaccine, mask, room],
             ["01a451cc344368", age, infected, vaccine, mask, room],
             ["01a451cc34c20d", age, infected, vaccine, mask, room],
             ["01a451cd100948", age, infected, vaccine, mask, room],
             ["01a451cc169025", age, infected, vaccine, mask, room],
             ["01a451cc169028", age, infected, vaccine, mask, room],
             ["01a451cc16900d", age, infected, vaccine, mask, room],
             ["01a451cc16900d", age, infected, vaccine, mask, room],
             ["01a451cc169040", age, infected, vaccine, mask, room]]
        with open('./../data/test3/server_test.csv', 'w') as f:
            writer = csv.writer(f)
            writer.writerows(l)
            s = "01a4511690"  
            start_int = 4011
            for i in range(1430):
                writer.writerow([s+str(start_int), age, infected, vaccine, mask, room])
                start_int += 1
    elif type == "client_0":
        with open('./../data/test3/client-24-22-0-test.csv', 'w') as f:
            writer = csv.writer(f)
            for i in range(1439):
                writer.writerow(["00000000000000", age, infected, vaccine, mask, room])
    elif type == "client_1":
        with open('./../data/test3/client-24-22-1-test.csv', 'w') as f:
            writer = csv.writer(f)
            writer.writerow(["01a451cc305925", age, infected, vaccine, mask, room])
            for i in range(1438):
                writer.writerow(["11111111111111", age, infected, vaccine, mask, room])
    elif type == "client":
        with open('./../data/test3/client-24-22-'+cli_num+'-test.csv', 'w') as f:
            writer = csv.writer(f)
            for i in range(1439):
                writer.writerow([cli_num*14, age, infected, vaccine, mask, room])
    else:
        raise Exception('Error! Please select a correct type{server,client_0,client_1}.')

create_hashed_data("server", 1, 1, 1, 1, 0, "0")
create_hashed_data("client_0", 1, 0, 0, 0, 0, "0")
create_hashed_data("client_1", 1, 0, 0, 0, 0, "0")
create_hashed_data("client", 2, 1, 3, 0, 1, "2")
create_hashed_data("client", 2, 1, 1, 0, 0, "3")
create_hashed_data("client", 3, 1, 2, 1, 0, "4")
create_hashed_data("client", 4, 0, 2, 1, 0, "5")
