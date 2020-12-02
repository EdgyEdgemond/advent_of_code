from contextlib import closing


def get_lines(day, test=False, type_conv=str):
    fn = "tests/day{}" if test else "inputs/day{}"
    with closing(open(fn.format(day))) as f:
        return [type_conv(line.strip()) for line in f.readlines()]
