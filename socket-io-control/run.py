#!/usr/bin/env python3
import subprocess
import threading
import os 

os.chdir('./server')


process = subprocess.Popen(
        ["node", "index.js"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)

os.chdir('../client')

for i in range(1, 6):
    command = f"node index.js {i} 5 2 5 50".split()
    process = subprocess.Popen(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    process.communicate()
