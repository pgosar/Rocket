#!/usr/bin/env python3
import numpy as np
import matplotlib.pyplot as plt

sleep_grid = np.genfromtxt("./logs/sleep-time.txt", delimiter=',', dtype="float64", skip_header=True)
out_degree_grid = np.genfromtxt("./logs/out-degree.txt", delimiter=',', dtype="float64", skip_header=True)
message_length_grid = np.genfromtxt("./logs/message-length.txt", delimiter=',', dtype="float64", skip_header=True)


grids = [sleep_grid, out_degree_grid, message_length_grid]
names = ["mean_sleep", "out_degree", "message_length"]
y_col = 5
x_cols = [4, 3, 6]
x_labels = ['Inter-message sleep time (ms)', 'Out-degree', 'Message length (chars)']
y_lims=[130, 100, 100]
titles=["Sleep Time Experiment", "Out Degree Experiment", "Message Length Experiment"]
colors=["red", "green", "blue"]

for i, grid in enumerate(grids):
    plt.plot(grids[i][:,x_cols[i]], grids[i][:,y_col], color=colors[i], marker='x')
    plt.ylabel('Client Servicing Time (ms)')
    plt.xlabel(x_labels[i])
    plt.title(titles[i])
    ax = plt.gca()
    ax.set_ylim([60, y_lims[i]])
    plt.gcf().savefig(f'rust-{names[i]}.png', dpi=200)
    plt.clf()
