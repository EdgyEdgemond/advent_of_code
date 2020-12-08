import re

from util import get_lines

bags = {}

contain_re = re.compile(r"(\d+) (\w+) (\w+) (\w+)?\.*")
class Bag:
    def __init__(self, name, contains):
        self.name = name
        contains = contains.split(", ")
        contains = [contain_re.match(c) for c in contains]
        contains = [(int(c[1]), "{} {} bag".format(c[2], c[3])) for c in contains if c]
        self.contains = contains
    
    def can_contain(self, name):
        for count, bag in self.contains:
            if bag == name or bags[bag].can_contain(name):
                return True

        return False
    
    def bag_count(self):
        total = 0
        for count, bag in self.contains:
            total += count + (count * bags[bag].bag_count())

        return total

lines = get_lines(
    day=7,
    part=2,
    test=True,
    type_conv=lambda x: x.split(" contain "),
)

for line in lines:
    bags[line[0][:-1]] = Bag(*line)

# Part 1
print(sum([bag.can_contain("shiny gold bag") for bag in bags.values()]))

# Part 2
print(bags["shiny gold bag"].bag_count())
