import subprocess
import shlex
import threading


def run_command(command):
    command = shlex.split(command)
    process = subprocess.Popen(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    time = [0.0]

    def check_output():
        elapsed_time = 0
        for line in process.stdout:
            print(line.lower(), end='')
            if "total time" in line.lower():
                elapsed_time = float(line.split(":")[1].strip().rstrip('ms'))
                process.terminate()
                break
        time[0] = elapsed_time
    output_thread = threading.Thread(target=check_output)
    output_thread.start()
    process.wait()
    output_thread.join()
    stderr_output, _ = process.communicate()
    print(stderr_output, end='')
    return time[0]


commands = []
with open("commands.txt", "r") as file:
    commands = [line.strip() for line in file.readlines()]

with open("times.txt", "w") as file:
    for command in commands:
        print(f"Running command: {command}")
        file.write(f"{run_command(command)}\n")
