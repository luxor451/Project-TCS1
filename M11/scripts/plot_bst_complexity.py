#!/usr/bin/env python3

import matplotlib.pyplot as plt
import matplotlib.lines as mlines
import numpy as np
import sys

data = np.genfromtxt('./results/data_complexity_bst_worst_case.csv' if len(sys.argv) == 1 else sys.argv[1],
                     delimiter=',',
                     names=['nb', 'insert', 'delete'])

fig, axes = plt.subplots(1,1)

axes.set_title("Time to insert/delete all nodes in/from BST")
axes.set_xlabel('# of nodes')
axes.set_ylabel('time')

red_line_legend = mlines.Line2D([], [], color='red',
                                label='insert')

blue_line_legend = mlines.Line2D([], [], color='blue',
                                 label='delete')

plt.legend(handles=[red_line_legend, blue_line_legend])

# time to insert
axes.plot(data['nb'], data['insert'], c='r', label='insert')

# time to extract
axes.plot(data['nb'], data['delete'], c='b', label='delete')

plt.show()
