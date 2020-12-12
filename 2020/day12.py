from util import get_lines

lines = get_lines(
    day=12,
    test=False,
    type_conv=lambda x: (x[0], int(x[1:])),
)


def question_one(lines):
    current = [0, 0]
    dirs = ['E', 'S', 'W', 'N']
    movements = [(1, 0), (0, -1), (-1, 0), (0, 1)]
    facing = 'E'

    for action, amount in lines:
        if action == 'F':
            action = facing

        if action in dirs:
            i = dirs.index(action)
            move = movements[i]
            current[0] += move[0] * amount
            current[1] += move[1] * amount

        if action in ['L', 'R']:
            i = dirs.index(facing)
            i = i + (amount // 90) if action == 'R' else i - (amount // 90)
            i = i % 4
            facing = dirs[i]

    print(abs(current[0]) + abs(current[1]))


def question_two(lines):
    current = [0, 0]
    waypoint = [10, 1]

    move_waypoint = {
        'E': (1, 0),
        'W': (-1, 0),
        'S': (0, -1),
        'N': (0, 1),
    }
    for action, amount in lines:
        if action == 'F':
            current[0] += waypoint[0] * amount
            current[1] += waypoint[1] * amount

        if action in move_waypoint:
            move = move_waypoint[action]
            waypoint[0] += move[0] * amount
            waypoint[1] += move[1] * amount

        if action in ['L', 'R']:
            move = amount if action == 'R' else 360 - amount
            rotate = move // 90
            if rotate == 0:
                continue
            if rotate == 1:
                waypoint[0], waypoint[1] = waypoint[1], waypoint[0] * -1
            if rotate == 2:
                waypoint[0], waypoint[1] = waypoint[0] * -1, waypoint[1] * -1
            if rotate == 3:
                waypoint[0], waypoint[1] = waypoint[1], waypoint[0] * -1
                waypoint[0], waypoint[1] = waypoint[1], waypoint[0] * -1
                waypoint[0], waypoint[1] = waypoint[1], waypoint[0] * -1

    print(abs(current[0]) + abs(current[1]))


question_one(lines)
question_two(lines)
