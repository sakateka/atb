# Uses python3
import sys


def optimal_summands(n):
    summands = [1, ]
    n -= 1
    while n > summands[-1]:
        summands.append(summands[-1] + 1)
        n -= summands[-1]

    for i in range(len(summands) - 1, -1, -1):
        if n > 0:
            summands[i] += 1
            n -= 1
    if n > 0:
        summands[-1] += n
    return summands


if __name__ == '__main__':
    input = sys.stdin.read()
    n = int(input)
    summands = optimal_summands(n)
    print(len(summands))
    for x in summands:
        print(x, end=' ')
    print()
