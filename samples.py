import matplotlib.pyplot as plt

with open('src-tauri/samples.csv', 'r') as f:
    data = f.readlines()

def in_range(x, y, z):
    x = float(x.strip())
    return x > y and x < z

# create time graph of data
# TODO is this already sampled. how about bitdepth. dont think so
# i mean, it has to sample it, but its still a lot. maybe not
# TODO set tick marks
print(f"{len(data)=} {len(data)/5}")
# it is sampled. not sure about bit depth though
sample_rate = 1
upper = 0.1
lower = 0.000001
# oh the rouding of float round(float(x.strip()))
# TODO make value better. round on output. but dont need to maybe
value = [float(x.strip()) for x in data[1:] if in_range(x, lower, upper) or in_range(x, -upper, -lower)][::sample_rate][300:400]
time = [ x for x in range(len(value)) ]
print(len(value))
print(f"{max(value)=} {min(value)=}")
plt.plot(time, value)
# plt.scatter(time, value)
plt.show()
