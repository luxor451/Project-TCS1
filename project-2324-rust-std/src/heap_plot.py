# need a heap_correctness.csv file in the same directory as the toml file
import csv
import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit
import math

# Function to fit to the data for n log n
def func_nlogn(n, a, b):
    return a * n * np.log(n) + b

# Function to fit to the data for n^2
def func_n2(n, a, b):
    return a * n**2 + b

# Read the CSV file
with open('./heap_correctness.csv', 'r') as f:
    reader = csv.reader(f)
    next(reader)  # Skip the header row
    data = list(reader)

# Split the data into separate lists
nb, time_to_insert, time_to_extract = zip(*data)

# Convert the data to integers
nb = list(map(int, nb))
time_to_insert = list(map(int, time_to_insert))
time_to_extract = list(map(int, time_to_extract))

# Fit the data to the n log n function
popt_insert_nlogn, _ = curve_fit(func_nlogn, nb, time_to_insert)
popt_extract_nlogn, _ = curve_fit(func_nlogn, nb, time_to_extract)

# Fit the data to the n^2 function
popt_insert_n2, _ = curve_fit(func_n2, nb, time_to_insert)
popt_extract_n2, _ = curve_fit(func_n2, nb, time_to_extract)

popt_insert_n2[0] *= 5
popt_extract_n2[0] *= 5
# Generate x values for the fitted function
x_fit = np.linspace(min(nb), max(nb), 1000)

# Plot the data
plt.plot(nb, time_to_insert, 'bo', label='Time to Insert')
plt.plot(nb, time_to_extract, 'ro', label='Time to Extract')
plt.plot(x_fit, func_nlogn(x_fit, *popt_insert_nlogn), 'b-', label='Fit with nln(n): Insert')
plt.plot(x_fit, func_nlogn(x_fit, *popt_extract_nlogn), 'r-', label='Fit with nln(n): Extract')
plt.plot(x_fit, func_n2(x_fit, *popt_insert_n2), 'b--', label='Fit with n^2: Insert')
plt.plot(x_fit, func_n2(x_fit, *popt_extract_n2), 'r--', label='Fit with n^2: Extract')
plt.xlabel('Number of Nodes')
plt.ylabel('Time (nanoseconds)')
plt.legend()
plt.show()