from util import get_lines

def question_one(values: list[str]) -> int:
    sum_ = 0
    for input_ in values:
        val = ""
        for c in input_:
            if c.isdigit():
                val += c
                break
        for c in reversed(input_):
            if c.isdigit():
                val += c
                break
        sum_ += int(val)
    return sum_

def question_two(values: list[str]) -> int:
    sum_ = 0
    replacements = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "fo4r"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "ni9e"),
    ]
    for input_ in values:
        val = ""
        for find, replace in replacements:
            input_ = input_.replace(find, replace)
        for c in input_:
            if c.isdigit():
                val += c
                break
        for c in reversed(input_):
            if c.isdigit():
                val += c
                break
        sum_ += int(val)
    return sum_

def run():
    values = get_lines(1)
    # values = get_lines(1, test=True, part=1)
    print("D1Q1: ", question_one(values))
    # values = get_lines(1, test=True, part=2)
    print("D1Q2: ", question_two(values))


if __name__ == "__main__":
    run()


def test_question_one(benchmark):
    values = get_lines(1)
    benchmark(question_one, values)


def test_question_two(benchmark):
    values = get_lines(1)
    benchmark(question_two, values)
