#!/usr/bin/python3

import random
import sys
from subprocess import PIPE, Popen, check_output

print(check_output(["rustc", "-O", "fib_huge_n_modulo_m.rs"]))
cmd_py = ["python", "../fibonacci_huge.py"]
cmd_rs = ["./fib_huge_n_modulo_m"]


def test(n, m):
    data = '{} {}\n'.format(n, m).encode('utf8')

    p = Popen(cmd_py, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_py = p.communicate(input=data)[0]

    p = Popen(cmd_rs, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_rs = p.communicate(input=data)[0]
    assert out_py == out_rs, "BUG: py: '{}', rs: '{}'".format(out_py, out_rs)
    sys.stdout.write("F_{} mod {} - OK = {}".format(n, m, out_rs.decode('utf8')))


test(1, 2)
test(1, 999)

while True:
    num = random.randint(1, 100000)
    mod = random.randint(2, 1000)
    test(num, mod)
