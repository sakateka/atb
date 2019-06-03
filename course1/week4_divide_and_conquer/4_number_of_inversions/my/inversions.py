# Uses python3
import sys


def merge(left, right, inv=0):
    ret = []

    while left and right:
        if left[0] > right[0]:
            inv += len(left)
            ret.append(right.pop(0))
        else:
            ret.append(left.pop(0))

    ret.extend(left)
    ret.extend(right)

    return ret, inv


def inversions(array):
    arr_len = len(array)
    if arr_len == 1:
        return array, 0

    l1, inv1 = inversions(array[:arr_len//2])
    l2, inv2 = inversions(array[arr_len//2:])

    return merge(l1, l2, inv1+inv2)


if __name__ == '__main__':
    input = sys.stdin.read()
    n, *a = list(map(int, input.split()))
    _, b = inversions(a)
    print(b)
