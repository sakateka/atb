#!/usr/bin/python3

import random
import sys
from subprocess import PIPE, Popen, check_output

print(check_output(["rustc", "-O", sys.argv[1]+".rs"]))
cmd_py = ["python", "../fibonacci_partial_sum.py"]
cmd_rs = ["./"+sys.argv[1]]


def test(n, m):
    data = '{} {}\n'.format(n, m).encode('utf8')

    p = Popen(cmd_py, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_py = p.communicate(input=data)[0].decode('utf8').strip()

    p = Popen(cmd_rs, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_rs, err_rs = p.communicate(input=data)
    out_rs = out_rs.decode('utf8').strip()
    err_rs = err_rs.decode('utf8').strip()
    if err_rs:
        print(err_rs)
    assert out_py == out_rs, "sum(F_{} ... F_{}) mod 10 - BUG: py: '{}', rs: '{}'".format(n, m, out_py, out_rs)
    return n, m, int(out_rs)


for g in range(4):
    for m in range(101):
        n = g
        if n == 3:
            n = m
        if n > m:
            n, m = m, g
        print("sum(F_{} ... F_{}) mod 10 - OK = {}".format(*test(n, m)))

while True:
    n = random.randint(1, 100000)
    m = random.randint(1, 100000)
    if m < n:
        m, n = n, m
    print("sum(F_{} ... F_{}) mod 10 - OK = {}".format(*test(n, m)))
