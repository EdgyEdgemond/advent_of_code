from contextlib import closing

with closing(open("inputs/day3")) as f:
    inputs = [l.strip() for l in f.readlines()]


# Part 1
for path in inputs:
    print(path)

    position = [0, 0]
    visited = {tuple(position)}

    for direction in path:
        if direction == ">":
            position[0] += 1
        elif direction == "<":
            position[0] -= 1
        elif direction == "^":
            position[1] += 1
        elif direction == "v":
            position[1] -= 1
        visited.add(tuple(position))
    print(len(visited))


# Part 2
for path in inputs:
    print(path)

    santa_position = [0, 0]
    robo_position = [0, 0]
    visited = {tuple(santa_position)}

    for i, direction in enumerate(path):
        if i % 2 == 0:
            position = santa_position
        else:
            position = robo_position

        if direction == ">":
            position[0] += 1
        elif direction == "<":
            position[0] -= 1
        elif direction == "^":
            position[1] += 1
        elif direction == "v":
            position[1] -= 1
        visited.add(tuple(position))

    print(len(visited))
