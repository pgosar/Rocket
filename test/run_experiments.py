#!/usr/bin/env python3

import sys
import subprocess
import os
import time
import fcntl


commands = []
with open("commands.txt", "r") as file:
    commands = [line.strip() for line in file.readlines()]
    file.close()

os.chdir("../socket-server/target/debug")

server_threads = sys.argv[1]
#output_log = sys.argv[6]



server_command = f"./socket_server -t {server_threads}"

server_process = subprocess.Popen(
        server_command.split(), universal_newlines=True)

os.chdir("../../../socket-client")
time.sleep(2)


for command_number, command in enumerate(commands):
    avg_line = ""
    avg_time = 0
    split_command = command.split()
    num_clients = int(split_command[split_command.index('-n') + 1])
    mean_sleep = float(split_command[split_command.index('-s') + 1])
    repeats = float(split_command[split_command.index('-r') + 1])
    message_length = float(split_command[split_command.index('-m') + 1])
    print(f"Job {command_number + 1} of {len(commands)} (expected latency {4 + (mean_sleep * repeats / 1000)}s)")
    for i in range(5):
        print(f"Iteration {i}")
        output_log = f"../test/logs/round-{command_number}.tmp"

        client_processes = []

        for j in range(num_clients):
            #client_command = f"cargo run -- -r 10 -n {num_clients} -o {out_degree} -s {sleep_ms} -f {output_log} -i {i}"
            client_command = f"{command} -i {j} -f {output_log}"
            client_process = subprocess.Popen(client_command.split(), universal_newlines=True)
            client_processes.append(client_process)
        
        for p in client_processes:
            p.wait()
            
        with open(output_log, "r") as f:
            lines = [line.strip() for line in f.readlines() if line.strip()]
            timings = [float(line.strip().split(',')[-1]) for line in lines]
            max_time = max(timings)
            mx = lines[timings.index(max_time)]
            avg_time += max_time
            if i == 4:
                avg_line = mx
        subprocess.run(["rm", output_log])

    avg_time /= 5.0
    avg_line = avg_line.split(',')
    avg_line[-1] = str(avg_time)
    avg_line = ','.join(avg_line)

    

    with open(f"../test/logs/experiment-{server_threads}-threads.txt", "a+") as f:
        f.write(f"{server_threads}," + avg_line + "," + str(message_length) + "\n")

print("Done!")
server_process.wait()