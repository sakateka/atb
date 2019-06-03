# Uses python3
import sys


def get_change(m):
    reminder = m
    coins = 0
    if reminder >= 10:
        coins += int(reminder / 10)
        reminder = reminder % 10
    if reminder >= 5:
        coins += int(reminder / 5)
        reminder = reminder % 5
    coins += reminder

    return coins


if __name__ == '__main__':
    m = int(sys.stdin.read())
    print(get_change(m))
