from util import get_lines

lines = get_lines(1, False, int)

# Part 1
for i in lines:
    if 2020 - i in lines:
        print(i * (2020 - i))

for i, item1 in enumerate(lines[:-3]):
    for j, item2 in enumerate(lines[i:-2]):
        for k, item3 in enumerate(lines[j:]):
            if item1 + item2 + item3 == 2020:
                print(item1 * item2 * item3)
