import numpy as np
import pandas as pd

# Define the parameters
f1 = 200
f2 = 1000
sampling_frequency = 20000
time_duration = 1

# Generate the time array
time = np.linspace(0, time_duration, int(sampling_frequency*time_duration))

# Calculate the amplitude for each frequency
amp1 = 5 * np.sin(2 * np.pi * f1 * time)
amp2 = 5 * np.sin(2 * np.pi * f2 * time)

# Combine the amplitudes into a single array
amp = amp1 + amp2

# Create a pandas DataFrame with time and amplitude values
df = pd.DataFrame({'Time': time, 'Amplitude': amp})

# Save the DataFrame as a CSV file
df.to_csv('sin_200_1000_f20000.txt', index=False, sep=',')
