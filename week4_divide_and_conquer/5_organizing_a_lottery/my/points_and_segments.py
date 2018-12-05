# Uses python3
import os
import sys
from collections import defaultdict, namedtuple
from functools import cmp_to_key
from itertools import chain

Dot = namedtuple('Dot', ['dot', 'type'])


def fast_count_segments(starts, ends, points):
    cnt = [0] * len(points)
    idx = defaultdict(list)
    for e in enumerate(points):
        idx[e[1]].append(e[0])
    start = 'l'
    end = 'r'
    point = 'p'

    dots = [d for d in chain(
        [Dot(i, start) for i in starts],
        [Dot(i, end) for i in ends],
        [Dot(i, point) for i in points],
    )]

    dots.sort(key=cmp_to_key(lambda a, b: a.dot - b.dot or ord(a.type) - ord(b.type)))
    n = 0
    for d in dots:
        if d.type == start:
            n += 1
        elif d.type == point:
            for point_idx in idx[d.dot]:
                cnt[point_idx] = n
        elif d.type == end:
            n -= 1
        try:
            assert n >= 0
        except AssertionError:
            print(dots)
            raise
    return cnt


def naive_count_segments(starts, ends, points):
    cnt = [0] * len(points)
    for i in range(len(points)):
        for j in range(len(starts)):
            if starts[j] <= points[i] <= ends[j]:
                cnt[i] += 1
    return cnt


def stress_test():
    if os.environ.get("__STRESS_BINARY_SEARCH"):
        import time
        import random
        left = -10**8
        right = 10**8
        max_len = 5000
        cases = [
                [[0, 0, 0, 0], [1, 2, 3, 4], [1, 2, 3, 4]],
                [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]],
                [[1, 2, 3, 4], [1, 2, 3, 4], [0, 1, 2, 5]],
                [[0, 1, 2, 3], [2, 3, 4, 5], [1, 2, 3, 4]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 1]],
                [[0, 0, 0, 0], [0, 0, 0, 0], [0]],
                [[0], [4], [-1, 0, 0, 3, 4, 5]],
                [[-100, -10, 0, 10, 100], [-100, -10, 0, 10, 100], [-100, -10]],
                [[0, 1, 2, 3], [3, 3, 3, 3], [0, 1, 2, 3]],
        ]
        while True:
            show_result = False
            try:
                (starts, ends, points) = cases.pop(0)
                show_result = True
            except IndexError:
                starts = [random.randint(left, right) for _ in range(random.randint(0, max_len)+1)]
                ends = [starts[i]+abs(random.randint(left+abs(starts[i]) + 1, right)) for i in range(len(starts))]
                points = [random.randint(left, right) for _ in range(random.randint(0, max_len)+1)]
            assert len(starts) == len(ends)

            start = time.time()
            n_ret = naive_count_segments(starts, ends, points)
            n_time = time.time() - start
            start = time.time()
            f_ret = fast_count_segments(starts, ends, points)
            f_time = time.time() - start
            msg = "n_ret({}), f_ret({})".format(n_ret, f_ret)
            assert n_ret == f_ret, msg
            if show_result:
                print(starts, ends, points, f_ret)
            print("naive={:0.3f}, fast={:0.3f}".format(n_time, f_time))


if __name__ == '__main__':
    stress_test()
    input = sys.stdin.read()
    data = list(map(int, input.split()))
    n = data[0]
    m = data[1]
    starts = data[2:2 * n + 2:2]
    ends = data[3:2 * n + 2:2]
    points = data[2 * n + 2:]
    # use fast_count_segments
    cnt = fast_count_segments(starts, ends, points)
    for x in cnt:
        print(x, end=' ')
