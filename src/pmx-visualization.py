# open pmx-data.txt
# in each line take three values and plot them
# first be x, second y, third y different color

import matplotlib.pyplot as plt
import numpy as np

# open file
f = open("pmx-data.txt", "r")

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
plt.title('PMX, 1000pop, 0.1 item chance, 0.05 mutation chance, 10.0 v_max, 0.1 v_min, 280_000 max_weight') 

# show plot
plt.show()

# save plot
# plt.savefig('pmx-visualization.png')



# end of file
