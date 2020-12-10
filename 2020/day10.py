from collections import defaultdict


from util import get_lines

lines = get_lines(
    day=10,
    test=False,
    type_conv=int,
)

lines = sorted(lines)
lines.append(max(lines) + 3)

# Part 1:
# c, d = 0, defaultdict(int)
#
# for j in lines:
#     d[j - c] += 1
#     c = j
k = 0

lines = set(lines)
def find_combos(j):
    global k
    k += 1
    print(k)
    if j == max(lines):
        return 1
    candidates = {i + 1 for i in range(j, j + 3)} & lines
    count = 0
    for c in candidates:
        count += find_combos(c) or 0
    return count
    # m = candidate[-1]
    # if m == max(lines):
    #     return [candidate]
    # combos = []
    # for i in range(3):
    #     if m + i + 1 in lines:
    #         combos += find_combos(candidate + [m + i + 1])
    # return combos

print(find_combos(0))
# combos = [[0]]
# final_combos = []
#
# for j in lines:
#     new_combos, keep_combos = [], []
#     for c in combos:
#         if j - c[-1] <= 3:
#             new_combos.append(c + [j])
#             keep_combos.append(c)
#
#     if j == max(lines):
#         combos = new_combos
#     else:
#         combos = new_combos + keep_combos
# print(len(combos))
# while True:
#     new_combos = []
#     for c in combos:
#         for i in range(3):
#             if c[-1] + i + 1 in lines:
#                 if c[-1] + i + 1 == max(lines):
#                     final_combos.append(c + [c[-1] + i + 1])
#                 else:
#                     new_combos.append(c + [c[-1] + i + 1])
#     if new_combos == []:
#         break
#     combos = new_combos
# print(len(final_combos))
