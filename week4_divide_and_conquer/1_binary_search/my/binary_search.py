# Uses python3
import os
import sys


def binary_search(a, x):
    left, right = 0, len(a)-1
    mid = 0
    while left <= right:
        mid = left + (right-left)//2
        if a[mid] == x:
            return mid
        elif a[mid] < x:
            left = mid + 1
        else:
            right = mid - 1
    return -1


def linear_search(a, x):
    for i in range(len(a)):
        if a[i] == x:
            return i
    return -1


def stress_test():
    if os.environ.get("__STRESS_BINARY_SEARCH"):
        import random
        upper_bound = 2**16
        seq = list(range(upper_bound+1))
        for x in seq:
            rand_int = random.randint(0, upper_bound)
            b_ret = binary_search(seq, rand_int)
            l_ret = linear_search(seq, rand_int)
            msg = "int({}): b_ret({}), l_ret({})".format(rand_int, b_ret, l_ret)
            assert b_ret == l_ret, msg
            print(msg, "Ok")


if __name__ == '__main__':
    stress_test()
    input = sys.stdin.read()
    data = list(map(int, input.split()))
    n = data[0]
    m = data[n + 1]
    a = data[1: n + 1]
    for x in data[n + 2:]:
        # replace with the call to binary_search when implemented
        print(binary_search(a, x), end=' ')
