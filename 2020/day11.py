# hash map of neighbours
# hash map of current pos
# loop, find which change based on neighbour map`
# loop apply change.
# repeat
from collections import defaultdict

from util import get_lines

lines = get_lines(
    day=11,
    test=False,
    type_conv=lambda x: list(x),
)

floor_map = {
    (r, c): col
    for r, row in enumerate(lines)
    for c, col in enumerate(row)
}

neighbours = {
    (r, c): [
        (i, j)
        for i in [r - 1, r, r + 1]
        for j in [c - 1, c, c + 1]
        if i >= 0 and j >= 0 and i < len(lines) and j < len(lines[0]) and (i, j) != (r, c)
    ]
    for r, row in enumerate(lines)
    for c, col in enumerate(row)
}


def check_pos(pos, floor_map):
    current = floor_map[pos]
    if current == ".":
        return "."

    occupied = sum([
        1
        for neighbour in neighbours[pos]
        if floor_map[neighbour] == "#"
    ])
    if occupied >= 4:
        return "L"
    elif occupied == 0:
        return "#"
    else:
        return current


flip_map = {
    pos: check_pos(pos, floor_map)
    for pos in floor_map
}

while floor_map != flip_map:
    floor_map = flip_map
    flip_map = {
        pos: check_pos(pos, floor_map)
        for pos in floor_map
    }

print(sum([p == "#" for p in floor_map.values()]))
