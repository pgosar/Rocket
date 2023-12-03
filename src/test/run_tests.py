import subprocess
import shlex
import threading


def run_command(command):
    command = shlex.split(command)
    num_disconnects = 0
    for i in range(len(command) - 1):
        if command[i] == "-n":
            num_disconnects = command[i + 1]
            break
    print(f"Expecting {num_disconnects} disconnect messages.")
    process = subprocess.Popen(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    disconnects = 0

    def check_output():
        nonlocal disconnects
        for line in process.stdout:
            print(line, end='')
            if "disconnecting client" in line.lower():
                disconnects += 1
            if disconnects == num_disconnects:
                print(
                    f"Reached {num_disconnects} disconnect messages. Terminating process.")
                process.terminate()
                break

    output_thread = threading.Thread(target=check_output)
    output_thread.start()
    process.wait()
    output_thread.join()
    stderr_output, _ = process.communicate()
    print(stderr_output, end='')


commands = []
with open("commands.txt", "r") as file:
    commands = [line.strip() for line in file.readlines()]

for command in commands:
    print(f"Running command: {command}")
    run_command(command)
