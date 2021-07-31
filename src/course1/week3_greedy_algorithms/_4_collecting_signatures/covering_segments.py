# Uses python3
import sys
from collections import namedtuple
from functools import cmp_to_key

Segment = namedtuple('Segment', 'start end')


def optimal_points(segments):
    points = []
    segments.sort(key=cmp_to_key(lambda a, b: a.start - b.start or a.end - b.end))
    points.append(segments[0].end)
    for s in segments:
        if points[-1] >= s.start and points[-1] <= s.end:
            continue
        if s.end < points[-1]:
            points[-1] = s.end
        else:
            points.append(s.end)
    return points


if __name__ == '__main__':
    input = sys.stdin.read()
    n, *data = map(int, input.split())
    segments = list(map(lambda x: Segment(x[0], x[1]), zip(data[::2], data[1::2])))
    points = optimal_points(segments)
    print(len(points))
    for p in points:
        print(p, end=' ')
    print()
