class A:
    def __init__(self, value):
        self.value = value

    def __add__(self, other):
        self.value += other.value
        return self

    def __truediv__(self, other):
        return self + other

    def __mul__(self, other):
        self.value *= other.value
        return self

    def __sub__(self, other):
        return self * other


def replace(add, mul, c):
    if c.isnumeric():
        return f'A({c})'
    elif c == '+':
        return add
    elif c == '*':
        return mul
    return c


lines = []
while True:
    try:
        lines.append(input())
    except:
        break


def solve(p, add, mul):
    total = 0
    for line in lines:
        s = ''.join([replace(add, mul, c) for c in line])
        r = eval(s).value
        total += r
    print(p, total)


solve('Part 1:', '+', '-')
solve('Part 2:', '/', '-')
