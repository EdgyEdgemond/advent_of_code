from copy import copy
from contextlib import closing


# with closing(open("tests/day2")) as f:
with closing(open("inputs/day2")) as f:
    lines = [[int(c) for c in l.strip().split(",")] for l in f.readlines()]


operations = {
    1: lambda a, b: a + b,
    2: lambda a, b: a * b,
    99: None,
}

# Tests.
# for line in lines:
#     i = 0
#     op = line[i]
#     while operations.get(op) is not None:
#         j, k, out = line[i + 1:i + 4]
#         a, b = line[j], line[k]
#         line[out] = operations.get(op)(a, b)
#         i += 4
#         op = line[i]
#     print(line)

# Part 1.

line = copy(lines[0])
line[1] = 12
line[2] = 2

def compute(line):
    i = 0
    op = line[i]
    while operations.get(op) is not None:
        j, k, out = line[i + 1:i + 4]
        a, b = line[j], line[k]
        line[out] = operations.get(op)(a, b)
        i += 4
        op = line[i]
    return line[0]

print(compute(line))

# Part 2.

for x in range(99):
    for y in range(99):
        line = copy(lines[0])
        line[1] = x
        line[2] = y

        out = compute(line)
        if out == 19690720:
            print(x * 100 + y)
