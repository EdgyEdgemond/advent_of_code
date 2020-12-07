import re

from util import get_lines

lines = get_lines(
    day=5,
    test=False,
    type_conv=lambda x: (x[:7], x[7:]),
)


# Part 1
def split(r):
    return r[:int(len(r)/2)], r[int(len(r)/2):]


rows = [_ for _ in range(128)]
cols = [_ for _ in range(8)]


seats = []
for line in lines:
    low, high = split(rows)
    for r in line[0]:
        if r == "F":
            low, high = split(low)
        else:
            low, high = split(high)

        if high == [] or low == []:
            try:
                row = high[0]
            except IndexError:
                row = low[0]
            break


    low, high = split(cols)
    for c in line[1]:
        if c == "L":
            low, high = split(low)
        else:
            low, high = split(high)

        if high == [] or low == []:
            try:
                seat = high[0]
            except IndexError:
                seat = low[0]
            break
    seats.append(row * 8 + seat)

print(max(seats))

# Part 2

available = range(128 * 8)

for i in available[1:-1]:
    if i not in seats and i - 1  in seats and i + 1 in seats:
        print(i)
