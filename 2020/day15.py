from collections import defaultdict

from util import get_lines

lines = get_lines(
    day=15,
    test=False,
    type_conv=lambda x: [int(_) for _ in x.split(",")],
)


def question(place, line):
    last = None
    seen = defaultdict(lambda: [None, None])

    for i in range(place):
        if i < len(line):
            current = line[i]
        else:
            l = seen[last]
            if l[0] is not None:
                current = l[1] - l[0]
            else:
                current = 0

        s = seen[current]
        seen[current] = s

        if s[1] is None:
            s[1] = i
        else:
            s[0], s[1] = s[1], i

        last = current
    print(last)


for line in lines:
    question(2020, line)
    # part 2
    question(30000000, line)
