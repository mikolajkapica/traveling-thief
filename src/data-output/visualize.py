# open pmx-data.txt
# in each line take three values and plot them
# first be x, second y, third y different color

import matplotlib.pyplot as plt
import numpy as np
import sys

type = sys.argv[1].split('-')[0]

# open file from argument
f = open(sys.argv[1], "r")

# read file
lines = f.readlines()

# close file
f.close()

# create empty lists
x = []
y = []
z = []

# read lines
for line in lines:
    # split line
    values = line.split()
    if values.__len__() < 10:
        continue
    x.append(float(values[1]))
    y.append(float(values[5]))
    z.append(float(values[9]))



# plot data
plt.plot(x, y, 'r')
plt.plot(x, z, 'b')

# set labels
plt.xlabel('generation')
plt.ylabel('fitness')

# set title
plt.title(type.upper() + ', 1000pop, 0.1 item chance, 0.05 mutation chance, 10.0 v_max, 0.1 v_min, 280_000 max_weight') 

# show plot
# plt.show()

# save plot
# image size to 1000x1000
plt.gcf().set_size_inches(10, 8)

# smaller text
plt.rcParams.update({'font.size': 8})
plt.savefig(type + '-visualization.png')



# end of file
