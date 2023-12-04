#!/usr/bin/env python3

import sys
import subprocess
import os
import time


commands = []
with open("node_commands.txt", "r") as file:
    commands = [line.strip() for line in file.readlines()]
    file.close()

os.chdir("../socket-io-control/server")

experiment_number = sys.argv[1]

server_command = f"node index.js"

server_process = subprocess.Popen(
        server_command.split(), universal_newlines=True)

os.chdir("../client")
time.sleep(2)

for command_number, command in enumerate(commands):
    client_processes = []
    output_log = f"../../test/logs/node-round-{command_number}.tmp"
    with open(output_log, 'a') as fhandle:
        split_command = command.split()
        num_clients = int(split_command[4])
        print(f"Job {command_number + 1} of {len(commands)}")
        for i in range(num_clients):
            client_command = f"{command} {i} {output_log}"
            client_process = subprocess.Popen(client_command.split(), stdout=fhandle, universal_newlines=True)
            client_processes.append(client_process)
        fhandle.close()
    for p in client_processes:
        p.wait()
    mx = 0
    with open(output_log, "r") as f:
        lines = [line.strip() for line in f.readlines() if line.strip()]
        timings = [float(line.split(',')[-1]) for line in lines]
        mx = lines[timings.index(max(timings))]
    with open(f"../../test/logs/node-experiment-{experiment_number}.txt", "a+") as f:
        f.write(str(mx) + "\n")
    subprocess.run(["rm", output_log])


print("Done!")
server_process.wait()