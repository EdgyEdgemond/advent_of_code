from util import get_lines

lines = get_lines(
    day=9,
    test=False,
    type_conv=int,
)

preamble = 25

for i in range(preamble, len(lines)):
    target = lines[i]
    options = lines[i-preamble:i]
    for o in options:
        if target - o in options and target - o != target:
            break
    else:
        print(target)
        break

for i, start in enumerate(lines):
    total = 0
    path = []
    path.append(start)
    for o in lines[i:]:
        total += o
        path.append(o)
        if total == target:
            print(min(path) +  max(path))
            break
        if total > target:
            break

    if total == target:
        break
