def generate_commands():
    repeats = [5, 10, 20, 50]
    num_clients = [5, 10, 20, 50]
    sleep_time_mean = [20, 50, 200, 1000]
    thread_counts = [1, 2, 4, 8, 16, 32]

    commands = []

    for repeat in repeats:
        for num_client in num_clients:
            outdegrees = [1, 2, num_client / 2, num_client - 1]
            for degree in outdegrees:
                for mean in sleep_time_mean:
                    for t in thread_counts:
                        command = (
                            f"cargo run -- "
                            f"-r {repeat} "
                            f"-n {num_client} "
                            f"-o {degree} "
                            f"-s {mean} "
                            f"-t {t}"
                        )
                        commands.append(command)

    return commands


commands = generate_commands()

with open("commands.txt", "w") as file:
    for command in commands:
        file.write(command + "\n")
