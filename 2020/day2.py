from util import get_lines

# Part 1
# class PasswordRule:
#     def __init__(self, rule, char):
#         self.rule = rule
#         self.char = char
#
#     def valid(self, password):
#         min_, max_ = [int(p) for p in self.rule.split("-")]
#         return min_ <= password.count(self.char) <= max_

# Part 2
class PasswordRule:
    def __init__(self, rule, char):
        self.rule = rule
        self.char = char

    def valid(self, password):
        pos1, pos2 = [int(p) - 1 for p in self.rule.split("-")]
        return ((password[pos1] == self.char) + (password[pos2] == self.char)) == 1

lines = get_lines(
    day=2,
    test=False,
    type_conv=lambda x: [
        PasswordRule(*p.split(" ")) if i == 0 else p
        for i, p in enumerate(x.split(": "))
    ],
)

valid = sum([r.valid(p) for r, p in lines])

print(valid)
