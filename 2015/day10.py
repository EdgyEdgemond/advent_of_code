test = ["1", 4]

# Part 1

actual = ["1113222113", 40]

# Part 2

actual = ["1113222113", 50]


def expand(s):
    prev = None
    res = []
    current = []
    for i in range(len(s)):
        if s[i] != prev:
            if prev is not None:
                res.append(current)
            current = []
        current.append(s[i])
        prev = s[i]
    res.append(current)
    return "".join(["{}{}".format(len(r), r[0]) for r in res])


s = test[0]
for i in range(test[1]):
    s = expand(s)
print(len(s))


s = actual[0]
for i in range(actual[1]):
    s = expand(s)
print(len(s))
