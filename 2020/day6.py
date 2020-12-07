import re

from util import get_lines

lines = get_lines(
    day=6,
    test=False,
    type_conv=set,
)

lines += [set()]

groups = []

# Part 1
group = set()
# Part 2
group = set('abcdefghijklmnopqrstuvwxyz')

for line in lines:
    if line != set():
        # Part 1
        # group |= line
        # Part 2
        group &= line

    else:
        groups.append(len(group))
        group = set('abcdefghijklmnopqrstuvwxyz')

print(groups)
print(sum(groups))
