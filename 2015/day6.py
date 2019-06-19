from contextlib import closing


with closing(open("inputs/day6")) as f:
    lines = [l.strip() for l in f.readlines()]


actions = ["turn on", "turn off", "toggle"]

lights = []

# Part 1
# for i in range(1000):
#     lights.append([])
#     for _ in range(1000):
#         lights[i].append(False)
#
#
# for l in lines:
#     for action in actions:
#         if l.startswith(action):
#             break
#     l = l.replace("{} ".format(action), "")
#     points = [p.split(",") for p in l.split(" through ")]
#
#     for i in range(int(points[0][0]), int(points[1][0]) + 1):
#         for j in range(int(points[0][1]), int(points[1][1]) + 1):
#             if action == "turn on":
#                 lights[i][j] = True
#             elif action == "turn off":
#                 lights[i][j] = False
#             else:
#                 lights[i][j] = not lights[i][j]

# Part 2
for i in range(1000):
    lights.append([])
    for _ in range(1000):
        lights[i].append(0)


for l in lines:
    for action in actions:
        if l.startswith(action):
            break
    l = l.replace("{} ".format(action), "")
    points = [p.split(",") for p in l.split(" through ")]

    for i in range(int(points[0][0]), int(points[1][0]) + 1):
        for j in range(int(points[0][1]), int(points[1][1]) + 1):
            if action == "turn on":
                lights[i][j] += 1
            elif action == "turn off":
                if lights[i][j] > 0:
                    lights[i][j] -= 1
            else:
                lights[i][j] += 2

print(sum([sum(lights[i]) for i in range(1000)]))
