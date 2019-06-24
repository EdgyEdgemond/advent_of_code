import re
from contextlib import closing


with closing(open("inputs/day14")) as f:
    lines = [l.strip() for l in f.readlines()]


r = re.compile(r"^(.*) can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds.")


reindeer = {}
for line in lines:
    m = r.match(line)

    reindeer[m[1]] = {
        "distance": int(m[2]),
        "time": int(m[3]),
        "rest": int(m[4]),
        "race": [],
        "points": 0,
    }


race_length = 2503


while any([len(r["race"]) < race_length for r in reindeer.values()]):
    for r in reindeer.values():
        if len(r["race"]) < race_length:
            r["race"].extend([r["distance"]] * r["time"])
            r["race"].extend([0] * r["rest"])


for r in reindeer.values():
    r["race"] = r["race"][:race_length]

# Part 1

winning_distance = 0
winner = None

for name, r in reindeer.items():
    d = sum(r["race"])
    if d > winning_distance:
        winning_distance = d
        winner = name


print(winner)
print(winning_distance)


# Part 2

for i in range(race_length):
    winning_distance = 0
    winners = []

    for name, r in reindeer.items():
        d = sum(r["race"][:i])
        if d > winning_distance:
            winning_distance = d
            winners = [name]
        elif d == winning_distance and winning_distance != 0:
            winners.append(name)

    for name in winners:
        reindeer[name]["points"] += 1


for name, r in reindeer.items():
    print(name, r["points"])
