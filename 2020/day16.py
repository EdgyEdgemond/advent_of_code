import re
from collections import defaultdict

from util import get_lines


rules_re = re.compile(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$")


lines = get_lines(
    day=16,
    test=False,
    type_conv=str,
)


def chunk(lines):
    ret = []
    cur = []
    for line in lines:
        if line == "":
            ret.append(cur)
            cur = []
        else:
            cur.append(line)
    ret.append(cur)
    return ret


class Rule:
    def __init__(self, text, s1, e1, s2, e2):
        self.text = text
        self.allowed = [i for i in range(s1, e1 + 1)] + [i for i in range(s2, e2 + 1)]

    def valid(self, value):
        return value in self.allowed


raw_rules, ticket, others = chunk(lines)

my_ticket = [int(f) for f in ticket[1].split(",")]

rules = []
for rule in raw_rules:
    m = rules_re.match(rule)
    rules.append(Rule(m[1], int(m[2]), int(m[3]), int(m[4]), int(m[5])))


allowed = []
for r in rules:
    allowed.extend(r.allowed)

blocked = [i for i in range(1000) if i not in allowed]

invalid_fields = []
valid_tickets = []
for ticket in others[1:]:
    fields = [int(i) for i in ticket.split(",") if int(i) in blocked]

    if fields == []:
        valid_tickets.append([int(f) for f in ticket.split(",")])
    else:
        invalid_fields.extend(fields)


# Part 1
print(sum(invalid_fields))

# Part 2
fields = defaultdict(list)

for ticket in valid_tickets:
    for i, f in enumerate(ticket):
        fields[i].append(f)

final_fields = {}

rule_sets = {}
for i, field_values in fields.items():
    rule_sets[i] = [
        rule
        for rule in rules
        if all([rule.valid(f) for f in field_values])
    ]

rule_sets = sorted([(pos, rules) for pos, rules in rule_sets.items()], key=lambda x: len(x[1]))

seen = set()
for pos, rules in rule_sets:
    for rule in rules:
        if rule.text not in seen:
            seen.add(rule.text)
            final_fields[rule.text] = pos

total = 1
for field, pos in final_fields.items():
    if field.startswith("departure"):
        total *= my_ticket[pos]


print(total)
