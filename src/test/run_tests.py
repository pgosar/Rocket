import subprocess
import shlex
import threading


def run_command(command):
    command = shlex.split(command)
    num_disconnects = 0
    for i in range(len(command) - 1):
        if command[i] == "-n":
            num_disconnects = int(command[i + 1])
            break
    print(f"Expecting {num_disconnects} disconnect messages.")
    process = subprocess.Popen(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
    time = [0]

    def check_output():
        disconnects = 0
        elapsed_time = 0
        for line in process.stdout:
            print(line.lower(), end='')
            if "disconnecting client" in line.lower():
                disconnects += 1
            if "total time" in line.lower():
                elapsed_time = float(line.split(":")[1].strip())
                print("Total Measured time:", elapsed_time)
            if disconnects == num_disconnects:
                print(
                    f"Reached {num_disconnects} disconnect messages. Terminating process.")
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
times = []
for command in commands:
    print(f"Running command: {command}")
    times.append(run_command(command))
with open("times.txt", "w") as file:
    for time in times:
        file.write(f"{time}\n")
