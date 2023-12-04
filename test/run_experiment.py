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

os.chdir("../socket-server")

server_threads = sys.argv[1]
#output_log = sys.argv[6]



server_command = f"cargo run -- -t {server_threads}"

server_process = subprocess.Popen(
        server_command.split(), universal_newlines=True)

os.chdir("../socket-client")
time.sleep(2)


for command_number, command in enumerate(commands):
    client_processes = []
    output_log = f"../test/logs/round-{command_number}.tmp"
    split_command = command.split()
    num_clients = int(split_command[split_command.index('-n') + 1])
    for i in range(num_clients):
        #client_command = f"cargo run -- -r 10 -n {num_clients} -o {out_degree} -s {sleep_ms} -f {output_log} -i {i}"
        client_command = f"{command} -i {i} -f {output_log}"
        client_process = subprocess.Popen(client_command.split(), universal_newlines=True)
        client_processes.append(client_process)
    
    for p in client_processes:
        p.wait()
        
    mx = 0
    with open(output_log, "r") as f:
        content = [int(line.strip()) for line in f.readlines() if line.strip()]
        mx = max(content)

    with open(f"../test/logs/experiment-{server_threads}-threads.txt", "a+") as f:
        f.write(str(mx) + "\n")
    #subprocess.run(["rm", output_log])

server_process.wait()