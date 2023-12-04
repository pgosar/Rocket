#!/usr/bin/env python3

import sys
import subprocess
import os
import time

os.chdir("../socket-server")

server_threads = int(sys.argv[1])
num_clients = int(sys.argv[2])
repeats = int(sys.argv[3])
out_degree = int(sys.argv[4])
sleep_ms = int(sys.argv[5])


server_command = f"cargo run -- -t {server_threads}"

server_process = subprocess.Popen(
        server_command.split(), universal_newlines=True)

os.chdir("../socket-client")

time.sleep(2)

#subprocess.run(["cargo", "build"]) 


for i in range(num_clients):
    client_command = f"cargo run -- -i {i} -r 10 -n {num_clients} -o {out_degree} -s {sleep_ms}"
    client_process = subprocess.Popen(
        client_command.split(), universal_newlines=True)

server_process.wait()
