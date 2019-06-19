from contextlib import closing


with closing(open("inputs/day8")) as f:
    lines = [l.strip() for l in f.readlines()]


def in_mem_len(s):
    s = s[1:-1]
    total = 0
    i = 0
    while i < len(s):
        if s[i] == "\\":
            if s[i + 1] == "x":
                i += 3
            else:
                i += 1

        total += 1
        i += 1
    return total


def encode_len(s):
    total = 2
    for c in s:
        if c in ['"', '\\']:
            total += 1
        total += 1
    return total


# Part 1

total = 0
for l in lines:
    total += len(l)
    total -= in_mem_len(l)

# Part 2

total = 0
for l in lines:
    total -= len(l)
    total += encode_len(l)

print(total)
