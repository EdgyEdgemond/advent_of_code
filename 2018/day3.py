import re
from contextlib import closing


with closing(open("inputs/day3")) as f:
    lines = [l.strip() for l in f.readlines()]


seen = set()
overlap = set()


r = re.compile(r".* @ (\d+),(\d+): (\d+)x(\d+)")


# Part 1
for line in lines:
    m = r.match(line)

    start = (int(m[1]), int(m[2]))
    size = (int(m[3]), int(m[4]))

    for i in range(size[0]):
        for j in range(size[1]):
            point = (start[0] + i, start[1] + j)

            if point in seen:
                overlap.add(point)

            seen.add(point)


print(len(overlap))


# Part 2
for line in lines:
    m = r.match(line)

    start = (int(m[1]), int(m[2]))
    size = (int(m[3]), int(m[4]))

    overlaps = False
    for i in range(size[0]):
        for j in range(size[1]):
            point = (start[0] + i, start[1] + j)

            if point in overlap:
                overlaps = True

    if not overlaps:
        print(line)
