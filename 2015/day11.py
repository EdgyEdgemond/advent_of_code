from contextlib import closing


with closing(open("inputs/day11")) as f:
    lines = [l.strip() for l in f.readlines()]


def check_has_consecutive_straight(s):
    for i in range(len(s) - 2):
        if ord(s[i]) + 1 == ord(s[i + 1]) and ord(s[i]) + 2 == ord(s[i + 2]):
            return True
    return False


def check_blacklist(s):
    blacklist = ["i", "o", "l"]
    for c in blacklist:
        if c in s:
            return False

    return True


def check_has_double_doubles(s):
    doubles = set()
    for i in range(len(s) - 1):
        if s[i] == s[i + 1] and s[i] not in doubles:
            doubles.add(s[i])
            i += 1

    return len(doubles) > 1


def validate(s):
    return all([
        check_has_consecutive_straight(s),
        check_blacklist(s),
        check_has_double_doubles(s),
    ])


def incr(s):
    s = list(s)
    for i in range(len(s) - 1, -1, -1):
        inc = 1
        if s[i] in ["i", "o", "l"]:
            inc = 2

        s[i] = chr(ord(s[i]) + inc) if ord(s[i]) < 122 else "a"

        if s[i] != "a":
            break
    return "".join(s)


for l in lines:
    print(l)
    while not validate(l):
        l = incr(l)
    print(l)
