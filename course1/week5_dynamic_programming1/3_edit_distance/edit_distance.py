# Uses python3
import os
import sys


def edit_distance(s, t):
    j_len = len(t)
    i_len = len(s)

    D = [[i] + [j for j in range(1, j_len+1)] for i in range(i_len+1)]

    for j in range(1, j_len+1):
        for i in range(1, i_len+1):
            ins = D[i][j - 1] + 1
            adel = D[i - 1][j] + 1
            match = D[i-1][j - 1]
            mis = D[i-1][j-1] + 1
            if s[i-1] == t[j-1]:
                D[i][j] = min(ins, min(adel, match))
            else:
                D[i][j] = min(ins, min(adel, mis))

    return D[i_len][j_len]


def edit_distance_test():
    if os.environ.get("__EDIT_DISTANCE_TEST"):
        distance = edit_distance("editing", "distance")
        assert 5 == distance, distance

        distance = edit_distance("short", "ports")
        assert 3 == distance, distance

        distance = edit_distance("ab", "ab")
        assert 0 == distance, distance

        print("test pass")
        sys.exit(1)


if __name__ == "__main__":
    edit_distance_test()
    print(edit_distance(input(), input()))
