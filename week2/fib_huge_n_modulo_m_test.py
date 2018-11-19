#!/usr/bin/python3

import random
import sys
from subprocess import PIPE, Popen, check_output

print(check_output(["rustc", "-O", "fib_huge_n_modulo_m.rs"]))
cmd_py = ["python", "../starters/week2_algorithmic_warmup/5_fibonacci_number_again/fibonacci_huge.py"]
cmd_rs = ["./fib_huge_n_modulo_m"]

num = 1
mod = 2
while True:
    data = '{} {}\n'.format(num, mod).encode('utf8')

    p = Popen(cmd_py, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_py = p.communicate(input=data)[0]

    p = Popen(cmd_rs, stdout=PIPE, stdin=PIPE, stderr=PIPE)
    out_rs = p.communicate(input=data)[0]
    assert out_py == out_rs, "BUG: py: '{}', rs: '{}'".format(out_py, out_rs)
    sys.stdout.write("F_{} mod {} - OK = {}".format(num, mod, out_rs.decode('utf8')))
    sys.stdout.flush()
    num = random.randint(1, 100000)
    mod = random.randint(2, 1000)
