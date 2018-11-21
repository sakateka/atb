#!/usr/bin/python3

import random
import sys
from subprocess import PIPE, Popen, check_output

print(check_output(["rustc", "-O", sys.argv[1]+".rs"]))
cmd_py = ["python", "../fibonacci_sum_last_digit.py"]
cmd_rs = ["./"+sys.argv[1]]


def test(n):
    data = '{}\n'.format(n).encode('utf8')

    p = Popen(cmd_py, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_py = p.communicate(input=data)[0].decode('utf8').strip()

    p = Popen(cmd_rs, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_rs = p.communicate(input=data)[0].decode('utf8').strip()
    assert out_py == out_rs, "sum(F_{}) mod 10 - BUG: py: '{}', rs: '{}'".format(n, out_py, out_rs)
    return n, int(out_rs)


it_is_period = False
for i in range(1000):
    n, res = test(i)
    if i > 0 and res == 0:
        it_is_period = True
    else:
        if it_is_period and res == 1:
            print("\033[32m", "="*30, "\033[0m")
        it_is_period = False
    print("sum(F_{}) mod 10 - OK = {}".format(n, res))

while True:
    num = random.randint(1, 100000)
    n, res = test(num)
    print("sum(F_{}) mod 10 - OK = {}".format(n, res))
