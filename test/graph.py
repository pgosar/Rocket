#!/usr/bin/env python3
import numpy as np
import matplotlib.pyplot as plt

rust_grid = np.genfromtxt("./rust_log.txt", delimiter=',', dtype="float64", skip_header=True)
node_grid = np.genfromtxt("./logs/node-experiment-5.txt", delimiter=',', dtype="float64", skip_header=True)

# 2 possibilities for out degree (2 graphs )
# 4 possibilities for num clients (4 lines per graph)
# 5 for threads (x axis)

out_degree_5_grid = rust_grid[rust_grid[:, 3] == 5, :]
out_degree_2_grid = rust_grid[rust_grid[:, 3] == 2, :]
out_degrees = [2, 5]
grids = [out_degree_2_grid, out_degree_5_grid]

for deg, grid in enumerate(grids):
    five_clients = grid[grid[:, 1] == 5, :]
    ten_clients = grid[grid[:, 1] == 10, :]
    fifty_clients = grid[grid[:, 1] == 50, :]
    hundred_clients = grid[grid[:, 1] == 100, :]
    lines = [five_clients, ten_clients, fifty_clients, hundred_clients]
    colors = ["red", "green", "blue", "black"]
    labels = ["5 clients", "10", "50", "100"]
    for i, line in enumerate(lines):
        plt.plot(line[:,0], line[:,5], color=colors[i], label=labels[i], marker='x')
    plt.ylabel('Client Servicing Time (ms)')
    plt.xlabel('Tokio Threads')
    plt.legend(loc="best")
    plt.title(f"Out degree: {out_degrees[deg]}")
    ax = plt.gca()
    ax.set_ylim([50, 110])
    plt.gcf().savefig(f'rust-{out_degrees[deg]}.png', dpi=200)
    plt.clf()


colors = ["red", "blue", "green", "black"]
labels = ["1 OD", "2", "4", "8"]
lines = []
for i in [1, 2, 4, 8]:
    lines.append(node_grid[node_grid[:, 2] == i, :])
for i, line in enumerate(lines):
    plt.plot(line[:,0], line[:,4], color=colors[i], label=labels[i], marker='x')
plt.title("Socket.io performance")
plt.ylabel('Client Servicing Time (ms)')
plt.xlabel('Number of Clients')
plt.legend(loc="best")

plt.gcf().savefig(f'node.png', dpi=200)
plt.clf()
