import csv
import matplotlib.pyplot as plt

x = []
y = []

with open('dft_order.txt', 'r') as csvfile:
    plots = csv.reader(csvfile, delimiter=',')
    for row in plots:
        x.append(float(row[0]))
        y.append(float(row[1]))

plt.plot(x, y)
plt.xlabel('Data Length')
plt.ylabel('Execution Time [sec]')
plt.title('DFT Order Test ')
plt.legend()
plt.show()
