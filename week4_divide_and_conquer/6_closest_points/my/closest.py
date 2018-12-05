# Uses python3
import math
import os
import sys


def naive_minimum_distance(x, y):
    points = list(sorted(zip(x, y), key=lambda x: x[0]))
    return naive_minimum_distance_int(points)


def naive_minimum_distance_int(points, limit=None):
    min_distance = 10 ** 18
    for (i, p1) in enumerate(points):
        if limit:
            sub_list = points[i+1: min(len(points), i + limit)]
        else:
            sub_list = points[i+1:]
        for p2 in sub_list:
            distance = math.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)
            if distance < min_distance:
                min_distance = distance
    return min_distance


def minimum_distance_int(points):
    if len(points) <= 3:
        return naive_minimum_distance_int(points)

    points_len = len(points)
    center_idx = points_len//2
    left_min = minimum_distance_int(points[:center_idx])
    right_min = minimum_distance_int(points[center_idx:])

    left_idx = center_idx - 1
    center = points[center_idx][0]
    while center - points[left_idx][0] < left_min:
        left_idx -= 1
        if left_idx == 0:
            break

    right_idx = center_idx + 1
    while points[right_idx][0] - center < right_min:
        right_idx += 1
        if right_idx == points_len:
            break
    center_points = list(sorted(points[left_idx:right_idx], key=lambda x: x[1]))
    center_min = naive_minimum_distance_int(center_points, limit=7)
    return min(min(left_min, right_min), center_min)


def minimum_distance(x, y):
    points = list(sorted(zip(x, y), key=lambda x: x[0]))
    return minimum_distance_int(points)


def stress_test():
    if os.environ.get("__STRESS_BINARY_SEARCH"):
        import time
        from numpy.random import randint
        cases = [
            [[4, -2, -3, -1, 2, -4, 1, -1, 3, -4, -2],  [4, -2, -4, 3, 3, 0, 1, -1, -1, 2, 4], 1.414213],
            [[0, 3], [0, 4], 5.0],
            [[7, 1, 4, 7], [7, 100, 8, 7], 0],
            [[4, -2, -3, -1, 2, -4, 1, -1, 3, -4, -2],  [4, -2, -4, 3, 3, 0, 1, -1, -1, 2, 4], None],
            [[0, 3], [0, 4], None],
            [[7, 1, 4, 7], [7, 100, 8, 7], None],
        ]
        cmin = -10**9
        cmax = 10**9
        i = 0
        while True:
            i += 1
            try:
                x, y, expect = cases.pop(0)
            except IndexError:
                expect = None
                n = randint(2, 10**5)
                if i % 2 == 0:
                    n = n % 2**12
                x = list(set(randint(cmin, cmax, n)))
                y = list(randint(cmin, cmax, len(x)))
            print("problem_size={}".format(len(x)), end=' ')
            start = time.time()
            if len(x) <= 2**12:
                answer = naive_minimum_distance(x, y)
            n_time = time.time() - start
            if expect is None:
                start = time.time()
                expect, answer = answer, minimum_distance(x, y)
                f_time = time.time() - start
                print("naive_time={:0.3f} fast_time={:0.3f}".format(
                    n_time, f_time), end=' ')
                if len(x) > 2**12:
                    print("")
                    continue

            try:
                assert abs(expect - answer) < 0.001, (expect, answer)
            except AssertionError:
                print(x)
                print(y)
                raise
            print("{} =~ {} -> OK!".format(expect, answer))
        sys.exit(0)


if __name__ == '__main__':
    stress_test()
    input = sys.stdin.read()
    data = list(map(int, input.split()))
    n = data[0]
    x = data[1::2]
    y = data[2::2]
    print("{0:.9f}".format(minimum_distance(x, y)))
