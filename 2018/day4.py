import re
from contextlib import closing


with closing(open("tests/day4")) as f:
    lines = [l.strip() for l in f.readlines()]


for line in sorted(lines):
    print(line)
