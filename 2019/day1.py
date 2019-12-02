from contextlib import closing


# with closing(open("tests/day1")) as f:
with closing(open("inputs/day1")) as f:
    lines = [int(l.strip()) for l in f.readlines()]


# Part 1

fuel_required = sum([
    (module / 3) - 2
    for module in lines
])

print(fuel_required)

# # Part 2

total = 0
for module in lines:
    fuel_required = (module / 3) - 2
    module_total = 0
    while fuel_required > 0:
        module_total += fuel_required
        fuel_required = (fuel_required / 3) - 2
    total += module_total

print(total)
