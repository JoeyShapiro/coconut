import matplotlib.pyplot as plt

with open('src-tauri/samples.csv', 'r') as f:
    data = f.readlines()

def in_range(x, y, z):
    x = float(x.strip())
    return x > y and x < z

# create time graph of data
sample_rate = 100
value = [round(float(x.strip())) for x in data[1:] if in_range(x, 0.000001, 10) or in_range(x, -10, -0.000001)][::sample_rate]
time = [ x for x in range(len(value)) ]
print(len(value))
print(f"{max(value)=} {min(value)=}")
plt.plot(time, value)
plt.show()

