# Uses python3
import os
import sys


def get_majority_element_nlogn(a):
    a_len = len(a)
    if a_len == 1:
        return a[0]
    if a_len == 2:
        if a[0] == a[1]:
            return a[0]
        else:
            return -1

    mid = a_len//2
    left_part = a[:mid]
    left = get_majority_element_nlogn(left_part)
    right_part = a[mid:]
    right = get_majority_element_nlogn(right_part)

    if left == right:
        return left

    left_count = 0
    if left != -1:
        left_count = sum(1 for el in a if el == left)
    right_count = 0
    if right != -1:
        right_count = sum(1 for el in a if el == right)

    major, el = (left_count, left) if left_count > right_count else (right_count, right)
    if major > mid:
        return el
    return -1


def get_majority_element_liner_space_big_o_1(a):
    count = 0
    candidate = None
    for num in a:
        if count == 0:
            candidate = num
        count += 1 if candidate == num else -1

    count = 0
    for num in a:
        count += 1 if candidate == num else 0
    if count > len(a)//2:
        return candidate

    return -1


def get_majority_element_liner(a):
    from collections import defaultdict
    count = defaultdict(int)
    majority = (len(a))//2 + 1
    for i in a:
        count[i] += 1
    for k in count:
        if count[k] >= majority:
            return k
    return -1


def get_majority_element_naive(a):
    for i in a:
        count = 0
        for j in a:
            if j == i:
                count += 1
        if count > len(a)//2:
            return i
    return -1


def stress_test():
    if os.environ.get("__STRESS_TEST"):
        import random
        import time
        manual_tests = [
            [1],
            [2, 2],
            [3, 3, 3],
            [1, 2, 3],
            [1, 2, 3, 4],
            [1, 2, 3, 1],
            [1, 2, 2, 2],
            [2, 1, 2, 2],
            [2, 2, 1, 2],
            [2, 2, 2, 1],
            [2, 2, 2, 1, 1],
            [2, 2, 2, 1, 1, 1],
            [1, 1, 2, 2, 2],
        ]
        while True:
            seq = [random.randint(0, 2**16) for x in range(random.randint(0, 2**8))]
            seq += [random.randint(0, 2**16)] * random.randint(0, 2**8)
            try:
                seq = manual_tests.pop()
            except IndexError:
                pass

            start = time.time()
            nm = get_majority_element_naive(seq)
            print("naive time={:0.3f}".format((time.time() - start) * 1000))
            start = time.time()
            lm = get_majority_element_liner(seq)
            print("liner time={:0.3f}".format((time.time() - start) * 1000))
            start = time.time()
            dm = get_majority_element_nlogn(seq)
            print("divide time={:0.3f}".format((time.time() - start) * 1000))
            start = time.time()
            bm = get_majority_element_liner_space_big_o_1(seq)
            print("liner with space O(1) time={:0.3f}".format((time.time() - start) * 1000))

            msg = "seq(len={}): nm({}) {{}} lm({}) {{}} dm({}) {{}} bm({})".format(
                len(seq), nm, lm, dm, bm)

            try:
                assert nm == lm, msg.format("!=", "??", "??")
                assert lm == dm, msg.format("==", "!=", "??")
                assert dm == bm, msg.format("==", "==", "!=")
                print(msg.format("==", "==", "=="), "ok")
            except AssertionError:
                print(seq)
                raise
            # assert l_majority == d_majority


if __name__ == '__main__':
    stress_test()
    input = sys.stdin.read()
    _, *a = list(map(int, input.split()))
    if get_majority_element_nlogn(a) != -1:
        print(1)
    else:
        print(0)
