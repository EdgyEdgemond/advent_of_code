from contextlib import closing


lines = [
    "+1, +1, +1",
    "+1, +1, -2",
    "-1, -2, -3",
]


with closing(open("inputs/day1")) as f:
    lines = [l.strip() for l in f.readlines()]


# Part 1
frequency = 0
for line in lines:
    changes = line.split(", ")
    for change in changes:
        delta = int(change[1:])
        if change[0] == "+":
            frequency += delta
        else:
            frequency -= delta

print(frequency)

# Part 2
seen_frequencies = set()

frequency = None
while frequency not in seen_frequencies:
    for line in lines:
        if frequency is not None:
            seen_frequencies.add(frequency)
        else:
            frequency = 0

        changes = line.split(", ")
        for change in changes:
            delta = int(change[1:])
            if change[0] == "+":
                frequency += delta
            else:
                frequency -= delta

        if frequency in seen_frequencies:
            break

print(frequency)
