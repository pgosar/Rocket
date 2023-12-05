#!/usr/bin/env python3
import math
def generate_commands():
    #repeats = [5, 50, 500] #[5, 10, 20, 50]
    repeats = [50]
    num_clients =  [50] #[5, 10, 20, 50]
    sleep_time_mean = [0] #[2, 5, 10, 15, 20, 25, 50, 100] #[20, 50, 200, 1000]
    #thread_counts = [1, ] #[1, 2, 4, 8, 16, 32]
    message_length = [10]#[10, 25, 50, 100, 250, 500]
    degree = [1, 2, 3, 4, 5, 8, 10, 12, 14, 16, 18, 20, 25]

    commands = []

    for mean in sleep_time_mean:
        for repeat in repeats:
            for num_client in num_clients:
                for deg in degree:
                #for degree in [1, 2, math.ceil(num_client / 10), num_client // 5]:
                #for t in thread_counts + [num_clients * 2]:
                    command = (
                        f"./target/debug/socket-client "
                        f"-r {repeat} "
                        f"-n {num_client} "
                        f"-o {deg} "
                        f"-s {mean} "
                        f"-m {message_length[0]}"
                    )
                    commands.append(command)

    return commands


commands = generate_commands()

with open("commands.txt", "w") as file:
    for command in commands:
        file.write(command + "\n")
