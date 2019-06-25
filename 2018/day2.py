from contextlib import closing


with closing(open("inputs/day2")) as f:
    lines = [l.strip() for l in f.readlines()]


# Part 1
alphabet = [chr(i) for i in range(97, 97 + 26)]


def extract_alpha_count(ident):
    ret = {}
    for c in alphabet:
        ret[c] = ident.count(c)

    return ret


double = 0
triple = 0
for line in lines:
    alpha_count = extract_alpha_count(line)
    double += 1 if any([v == 2 for v in alpha_count.values()]) else 0
    triple += 1 if any([v == 3 for v in alpha_count.values()]) else 0

print(double * triple)


def difference_count(ident, other):
    return sum([int(ident[i] != other[i]) for i in range(len(ident))])


found = False
for line in lines:
    if found:
        break
    for other_line in lines:
        if line != other_line and difference_count(line, other_line) == 1:
            print("".join([line[i] for i in range(len(line)) if line[i] == other_line[i]]))
            found = True
            break
