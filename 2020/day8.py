import re

from util import get_lines

cmd_re = re.compile(r"(acc|jmp|nop) ([+-]{1}\d*)")
lines = get_lines(
    day=8,
    test=False,
    type_conv=str,
)

orig_stack = []
for line in lines:
    m = cmd_re.match(line)
    orig_stack.append((m[1], int(m[2])))

# Part 2
for j in range(len(orig_stack)):
    cmd, x = orig_stack[j]

    stack = orig_stack[:]
    if cmd == "jmp":
        stack[j] = ("nop", x)
    elif cmd == "nop":
        stack[j] = ("jmp", x)
    else:
        continue

    # Part 1
    acc, i = 0, 0
    seen = set()

    while i < len(stack):
        cmd, x = stack[i]

        if i in seen:
            print(acc)
            break
        else:
            seen.add(i)

        if cmd == "acc":
            acc += x

        if cmd == "jmp":
            i += x
        else:
            i += 1
    else:
        print("success", acc)
        break
