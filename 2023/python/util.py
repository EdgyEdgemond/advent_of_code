from contextlib import closing

def get_lines(day, test=False, type_conv=str, part=None):
    if part and test:
        day = "{}.{}".format(day, part)
    fn = "../input/tests/day{}" if test else "../input/day{}"
    with closing(open(fn.format(day))) as f:
        return [type_conv(line.strip()) for line in f.readlines()]


