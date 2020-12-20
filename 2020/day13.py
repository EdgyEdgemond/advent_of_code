from util import get_lines

lines = get_lines(
    day=13,
    test=True,
    type_conv=str,
)

timestamp = int(lines[0])
routes = [int(x) if x != "x" else x for x in lines[1].split(",")]

def question_one(timestamp, routes):
    min_ = (0, 1000)

    for r in routes:
        if r == "x":
            continue

        wait = r - (timestamp % r)
        if wait < min_[1]:
            min_ = (r, wait)

    print(min_[0] * min_[1])


def question_two(routes):
    i = 0
    x = [r - j for j, r in enumerate(routes) if r != "x"]
    t =  1
    for r in x:
        t *= r
    print(t)
    # while True:
    #     if all((i + j) % r == 0 for j, r in enumerate(routes) if r != "x"):
    #         print(i)
    #         break
    #     i += 1
    
question_one(timestamp, routes)
question_two(routes)
