from contextlib import closing


with closing(open("inputs/day5")) as f:
    inputs = [l.strip() for l in f.readlines()]


def check_three_vowels(s):
    return sum([1 if i in ["a", "e", "i", "o", "u"] else 0 for i in s]) >= 3


def check_has_double(s):
    for i in range(len(s) - 1):
        if s[i] == s[i+1]:
            return True
    return False


def check_blacklist(s):
    blacklist = ["ab", "cd", "pq", "xy"]
    for i in blacklist:
        if i in s:
            return False

    return True


def check_has_repeated_pair(s):
    for i in range(len(s) - 2):
        if s[i:i+2] in s[i+2:]:
            return True
    return False


def check_has_sandwich(s):
    for i in range(len(s) - 2):
        if s[i] == s[i+2]:
            return True
    return False


part1_total = 0
part2_total = 0
for s in inputs:
    if all(
        [
            check_three_vowels(s),
            check_has_double(s),
            check_blacklist(s),
        ],
    ):
        part1_total += 1

    if all(
        [
            check_has_repeated_pair(s),
            check_has_sandwich(s),
        ],
    ):
        part2_total += 1

print(part1_total)
print(part2_total)
