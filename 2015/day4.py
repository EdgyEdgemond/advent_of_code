from contextlib import closing
from hashlib import md5


with closing(open("inputs/day4")) as f:
    inputs = [l.strip() for l in f.readlines()]


for seed in inputs:
    answer = 0
    found = False
    while not found:
        answer += 1
        source = "{}{}".format(seed, answer)

        m = md5()
        m.update(source.encode('utf-8'))
        # Part 1
        # found = m.hexdigest().startswith("00000")
        # Part 2
        found = m.hexdigest().startswith("000000")

    print(seed)
    print(answer)
