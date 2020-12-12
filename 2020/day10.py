from collections import defaultdict

from util import get_lines

lines = get_lines(
    day=10,
    test=False,
    type_conv=int,
)

lines = sorted(lines)
lines.append(max(lines) + 3)


def question_one(items):
    c, d = 0, defaultdict(int)

    for j in lines:
        d[j - c] += 1
        c = j

    print(c)


def question_two(items):
    cache = {0: 1}
    for item in items:
        cache[item] = (
            cache.get(item - 3, 0) +
            cache.get(item - 2, 0) +
            cache.get(item - 1, 0)
        )
    print(cache[items[-1]])


question_one(lines)
question_two(lines)
