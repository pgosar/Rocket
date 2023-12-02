def generate_commands():
    modes = ["c", "s"]
    repeats = ["1", "5", "10", "20", "50"]
    num_clients = ["2", "5", "10", "20", "50"]
    out_degrees = ["1", "3", "6", "15", "40"]
    sleep_time_mean = ["1", "2", "3", "5"]
    sleep_time_std = ["0", "1", "2", "3"]
    thread_counts = ["1", "2", "4", "8", "16", "32"]

    commands = []

    for mode in modes:
        for repeat in repeats:
            for num_client in num_clients:
                for out_degree in out_degrees:
                    for mean in sleep_time_mean:
                        for std in sleep_time_std:
                            for t in thread_counts:
                                command = (
                                    f"cargo run -- "
                                    f"-m {mode} "
                                    f"-r {repeat} "
                                    f"-n {num_client} "
                                    f"-o {out_degree} "
                                    f"-s {mean},{std}"
                                    f"-t {t}"
                                )
                                commands.append(command)

    return commands


commands = generate_commands()

with open("commands.txt", "w") as file:
    for command in commands:
        file.write(command + "\n")
