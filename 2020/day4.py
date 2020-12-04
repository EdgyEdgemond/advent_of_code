import re

from util import get_lines

lines = get_lines(
    day=4,
    test=False,
    type_conv=lambda x: x.split(' '),
)

required_keys = {
    'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid',
}

hcl_re = re.compile('^#[0-9a-f]{6}$')

rules = {
    'byr': lambda x: 1920 <= int(x) <= 2002,
    'iyr': lambda x: 2010 <= int(x) <= 2020,
    'eyr': lambda x: len(x) == 4 and 2020 <= int(x) <= 2030,
    'hgt': lambda x: (x.endswith('cm') and 150 <= int(x[:-2]) <= 193) or (x.endswith('in') and 59 <= int(x[:-2]) <= 76),
    'hcl': lambda x: hcl_re.match(x),
    'ecl': lambda x: x in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'},
    'pid': lambda x: len(x) == 9,
}

class Passport:
    def __init__(self, **kwargs):
        self.kwargs = kwargs

    def validate(self):
        keys = set(self.kwargs.keys())
        valid = required_keys - keys == set()

        for k, v in self.kwargs.items():
            if k in rules:
                if not rules[k](v):
                    valid = False
        return valid


passports = []
passport = []
for line in lines:
    if line == ['']:
        passports.append(
            Passport(**{kv.split(':')[0]: kv.split(':')[1] for kv in passport})
        )
        passport = []
    else:
        passport += line

passports.append(
    Passport(**{kv.split(':')[0]: kv.split(':')[1] for kv in passport})
)

print(sum([p.validate() for p in passports]))
