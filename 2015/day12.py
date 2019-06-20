import json
import copy
from contextlib import closing


with closing(open("tests/day12.2")) as f:
    lines = [l.strip() for l in f.readlines()]


def find_ints(o):
    ints = []
    if isinstance(o, list):
        for v in o:
            try:
                ints.extend(find_ints(v))
            except Exception:
                pass
    elif isinstance(o, dict):
        ints_copy = copy.copy(ints)
        try:
            for v in o.values():
                ints.extend(find_ints(v))
        except Exception:
            ints = ints_copy
    elif isinstance(o, int):
        ints.append(o)

    if o == "red":
        raise Exception("Ignore red")

    return ints


for l in lines:
    print(l)
    print(sum(find_ints(json.loads(l))))
