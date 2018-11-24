# Uses python3
import sys
from collections import namedtuple

IndexItem = namedtuple('IndexItem', ['cost', 'idx'])


def get_optimal_value(capacity, weights, values):
    value = 0.
    index = [IndexItem(values[idx] / weights[idx], idx) for idx in range(len(values))]
    index = sorted(index, key=lambda item: item.cost, reverse=True)
    for i in index:
        if capacity == 0:
            return value
        add_w = min(weights[i.idx], capacity)
        value += add_w * (values[i.idx] / weights[i.idx])
        weights[i.idx] -= add_w
        capacity -= add_w
    return value


if __name__ == "__main__":
    data = list(map(int, sys.stdin.read().split()))
    n, capacity = data[0:2]
    values = data[2:(2 * n + 2):2]
    weights = data[3:(2 * n + 2):2]
    opt_value = get_optimal_value(capacity, weights, values)
    print("{:.10f}".format(opt_value))
