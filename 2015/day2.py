from contextlib import closing


with closing(open("inputs/day2")) as f:
    boxes = [l.strip().split("x") for l in f.readlines()]

# Part 1
total = 0
for box in boxes:
    l, w, h = int(box[0]), int(box[1]), int(box[2])
    x = l * w
    y = l * h
    z = w * h
    slack = min([x, y, z])
    total += 2 * x + 2 * y + 2 * z + slack

print(total)

# Part 2
total = 0
for box in boxes:
    l, w, h = int(box[0]), int(box[1]), int(box[2])
    x = 2 * l + 2 * w
    y = 2 * l + 2 * h
    z = 2 * w + 2 * h
    bow = min([x, y, z])
    total += l * w * h + bow

print(total)
