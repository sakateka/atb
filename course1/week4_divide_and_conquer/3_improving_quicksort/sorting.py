# Uses python3
import os
import random
import sys

sys.setrecursionlimit(10000)


def partition3(a, left, right):
    x = a[left]
    j_left = left
    j_right = left
    for i in range(left + 1, right + 1):
        if a[i] <= x:
            j_right += 1
            a[i], a[j_right] = a[j_right], a[i]
            if a[j_right] < x:
                j_left += 1
                a[j_left], a[j_right] = a[j_right], a[j_left]
    a[left], a[j_left] = a[j_left], a[left]
    return j_left, j_right


def partition2(a, left, right):
    x = a[left]
    j = left
    for i in range(left + 1, right + 1):
        if a[i] <= x:
            j += 1
            a[i], a[j] = a[j], a[i]
    a[left], a[j] = a[j], a[left]
    return j


def randomized_quick_sort_partition3(a, left, right):
    if left >= right:
        return
    k = random.randint(left, right)
    a[left], a[k] = a[k], a[left]
    # use partition3
    lm, rm = partition3(a, left, right)
    randomized_quick_sort_partition3(a, left, lm - 1)
    randomized_quick_sort_partition3(a, rm + 1, right)


def randomized_quick_sort_partition2(a, left, right):
    if left >= right:
        return
    k = random.randint(left, right)
    a[left], a[k] = a[k], a[left]
    # use partition3
    m = partition2(a, left, right)
    randomized_quick_sort_partition2(a, left, m - 1)
    randomized_quick_sort_partition2(a, m + 1, right)


def stress_test():
    if os.environ.get("__STRESS_TEST"):
        import time
        manual_tests = [
            [1],
            [2, 2],
            [3, 3, 1],
            [1, 3, 3, 3, 3, 2],
        ]
        while True:
            if not manual_tests:
                seq1 = [random.randint(1, 10**9) for x in range(random.randint(1, 2**12))]
                seq1 += [random.randint(1, 10**9)] * random.randint(1, 2**12)
            else:
                seq1 = manual_tests.pop()

            seq2 = seq1.copy()
            seq3 = seq2.copy()
            right = len(seq2) - 1
            start = time.time()
            randomized_quick_sort_partition2(seq2, 0, right)
            print("sort part2 time={:0.3f}".format((time.time() - start) * 1000))
            start = time.time()
            randomized_quick_sort_partition3(seq3, 0, right)
            print("sort part3 time={:0.3f}".format((time.time() - start) * 1000))

            try:
                assert seq2 == seq3
            except AssertionError:
                for i, _ in enumerate(seq2):
                    if seq2[i] == seq3[i]:
                        continue
                    print("differ at {}".format(i))
                    print("seq1: {}".format(seq1))
                    print("seq2: " + ", ".join(map(str, seq2[:i])) + ", [31m" + str(seq2[i]) +
                          "[0m, " + ", ".join(map(str, seq2[i+1:])))
                    print("seq2: " + ", ".join(map(str, seq3[:i])) + ", [31m" + str(seq3[i]) +
                          "[0m, " + ", ".join(map(str, seq3[i+1:])))
                    break
                raise


def stress_test_rs():
    if os.environ.get("__STRESS_TEST_RS"):
        import time
        from subprocess import Popen, PIPE, check_output
        print(check_output(["rustc", "-O", "sorting.rs"]))
        while True:
            seq1 = [random.randint(1, 10**9) for x in range(random.randint(1, 2**17))]
            seq1 += [random.randint(1, 10**9)] * random.randint(1, 2**17)
            data = '{} {}\n'.format(len(seq1), " ".join(map(str, seq1))).encode('utf8')

            seq3 = seq1.copy()
            right = len(seq1) - 1
            start = time.time()
            randomized_quick_sort_partition3(seq3, 0, right)
            print("sort part3 time={:0.3f}".format((time.time() - start)))

            start = time.time()
            cmd = Popen(["./sorting"], stdout=PIPE, stdin=PIPE, stderr=PIPE)
            out_rs = cmd.communicate(input=data)[0].decode('utf8').strip()
            print("sort rs time={:0.3f}".format((time.time() - start)))

            out_py = " ".join(map(str, seq3))
            assert out_py == out_rs, "\npy: {}\n\nrs: {}".format(out_py, out_rs)


if __name__ == '__main__':
    stress_test_rs()
    stress_test()
    input = sys.stdin.read()
    n, *a = list(map(int, input.split()))
    randomized_quick_sort_partition3(a, 0, n - 1)
    for x in a:
        print(x, end=' ')
