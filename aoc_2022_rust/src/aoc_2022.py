
#%% day01 {{{

with open('./input/day01.txt', 'r') as f:
    elves = [0]
    for line in f.readlines():
        try:
            elves[-1] += int(line[:-1])
        except ValueError:
            elves.append(0)
print(f"day01a: {max(elves)}")
print(f"day01b: {max(sorted(elves)[-3:])}")

# }}}
#%% day03 {{{

SCORE = dict(zip("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",range(1,53)))


lines = open('./input/day03.txt').read().splitlines()

def eval_line(line:str):
    mid = len(line)// 2
    a, b = line[:mid], line[mid:]

    for ch in a:
        if ch in b:
            return SCORE[ch]


print(f"day03a: {sum(map(lambda l: eval_line(l), lines))}")


def chunks(lst, n):
    for i in range(0, len(lst), n):
        yield lst[i:i + n]


# }}}
#%% day05  {{{

import re
from copy import deepcopy

def split_at(lst: list, delim):
    i = [i for i, v in enumerate(lst) if v == delim][0]
    return lst[:i], lst[i+1:]

lines = open('./input/day05.txt').read().splitlines()
state_str, moves_str = split_at(lines, "")
state_str = [l[1::4] for l in state_str]

state: list[list] = [list() for _ in state_str[-1]]

for l in reversed(state_str[:-1]):
    for i, ch in enumerate(l):
        if ch != ' ':
            state[i].append(ch)

def move1(state: list[list], n, f, t):
    for _ in range(n):
        crate = state[f-1].pop()
        state[t-1].append(crate)

def move2(state: list[list], n, f, t):
    stack = []
    for _ in range(n):
        stack.append(state[f-1].pop())
    state[t-1].extend(reversed(stack))


move_re = re.compile(r"move (\d*) from (\d*) to (\d*)$")
state1 = state
state2 = deepcopy(state)
for l in moves_str:
    m = move_re.match(l)
    if m:
        n, f, t = [int(v) for v in m.groups()]
        move1(state1, n, f, t)
        move2(state2, n, f, t)


print("day05a:", "".join([pile[-1] for pile in state1]))
print("day05b:", "".join([pile[-1] for pile in state2]))

# }}}





