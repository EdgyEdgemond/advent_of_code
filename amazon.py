import re


def orderedJunctionBoxes(numberOfBoxes, boxList):
    # WRITE YOUR CODE HERE
    if numberOfBoxes == 1:
        return boxList

    box_list = [l.split() for l in boxList]
    for i in range(numberOfBoxes):
        for j in range(0, numberOfBoxes - i - 1):
            if check_gt(box_list[j], box_list[j + 1]):
                box_list[j], box_list[j + 1] = box_list[j + 1], box_list[j]
    return [" ".join(l) for l in box_list]
    

r = re.compile(r"(\d+)")

def check_gt(box, other):
    if len(box) > 1 and len(other) > 1:
        m = r.match(box[1])
        m2 = r.match(other[1])

        if m is None and m2 is not None:
            return False
        elif m is not None and m2 is None:
            return True
        elif m is not None and m2 is not None:
            return False

        for i in range(1, min(len(box), len(other))):
            print(i)
            if box[i] > other[i]:
                return True
            elif box[i] < other[i]:
                return False

    if len(box) > len(other):
        return True
    if box[0] > other[0]:
        return True
        
    return False


print(orderedJunctionBoxes(6, ["ykc 82 01", "eo first qpx", "09z cat dog", "06g 12 25 6", "az0 first qpx", "236 cat dog rabbit snake"]))
