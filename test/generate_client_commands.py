#!/usr/bin/env python3

def generate_commands():
    repeats = [5, 50, 500] #[5, 10, 20, 50]
    num_clients =  [10, 50, 100] #[5, 10, 20, 50]
    sleep_time_mean = [2, 5, 50] #[20, 50, 200, 1000]
    outdegrees = [2]
    #thread_counts = [1, 4] #[1, 2, 4, 8, 16, 32]

    commands = []

    for mean in sleep_time_mean:
        for degree in outdegrees:
            for repeat in repeats:
                for num_client in num_clients:
                    #for t in thread_counts + [num_clients * 2]:
                    command = (
                        f"./target/debug/socket-client "
                        f"-r {repeat} "
                        f"-n {num_client} "
                        f"-o {degree} "
                        f"-s {mean} "
                    )
                    commands.append(command)

    return commands


commands = generate_commands()

with open("commands.txt", "w") as file:
    for command in commands:
        file.write(command + "\n")
