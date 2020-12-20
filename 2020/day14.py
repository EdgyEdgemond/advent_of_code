import re
from collections import defaultdict

from util import get_lines

mem_re = re.compile(r"^mem\[(\d+)\] = (\d+)$")

lines = get_lines(
    day=14,
    test=False,
    part=2,
    type_conv=str,
)


class ValueMask:
    def __init__(self, mask):
        self.mask = {
            i: v
            for i, v in enumerate(mask)
            if v != "X"
        }

    def apply(self, value):
        binstr = list(format(value, '036b'))
        for i, v in self.mask.items():
            binstr[i] = v
        return int("".join(binstr), 2)


class AddressMask:
    def __init__(self, mask):
        self.one_mask = [
            i
            for i, v in enumerate(mask)
            if v == "1"
        ]
        self.float_mask = [
            i
            for i, v in enumerate(mask)
            if v == "X"
        ]

    def apply(self, value):
        binstr = list(format(value, '036b'))
        for i in self.one_mask:
            binstr[i] = "1"
        for i in self.float_mask:
            binstr[i] = "0"
        addresses = [binstr]

        for i in self.float_mask:
            new_addresses = []
            for address in addresses:
                new_address = address[:]
                new_address[i] = "1"
                new_addresses.append(new_address)
            addresses.extend(new_addresses)
        return [int("".join(address), 2) for address in addresses]


def question_one(lines):
    mem = defaultdict(int)

    for line in lines:
        if line.startswith("mask"):
            mask = ValueMask(line.split("mask = ")[1])
            continue
        m = mem_re.match(line)

        mem[m[1]] = mask.apply(int(m[2]))

    print(sum(mem.values()))


def question_two(lines):
    mem = defaultdict(int)

    for line in lines:
        if line.startswith("mask"):
            mask = AddressMask(line.split("mask = ")[1])
            continue
        m = mem_re.match(line)

        addresses = mask.apply(int(m[1]))
        for address in addresses:
            mem[address] = int(m[2])

    print(sum(mem.values()))


question_one(lines)
question_two(lines)
