from util import get_lines

lines = get_lines(
    day=3,
    test=False,
    type_conv=lambda x: list(x),
)

m = len(lines[0])

# Part 1
slopes = [(3, 1)]

# Part 2
slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]

trees = [0 for _ in slopes]
for s, slope in enumerate(slopes):
    i, j = 0, 0
    x, y = slope
    while True:
        i = (i + x) % m
        j += y

        try:
            trees[s] += lines[j][i] == "#"
        except IndexError:
            break

total = 1
for t in trees:
    total *= t
print(total)
