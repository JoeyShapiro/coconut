
numbs = {}
with open('output.txt') as f:
    for line in f:
        if line[0] in [ '1', '2', '3', '4' ]:
            numbs[line[0]] = numbs.get(line[0], 0) + 1

print(numbs)

numbs = { '3': 10578, '4': 258, '1': 190, '2': 184 }
