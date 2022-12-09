
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

from numpy.lib import math

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
#%% day07 {{{

from collections import defaultdict

lines = open("./input/day07.txt", 'rt').read().splitlines()
curdir = ["/"]
sizes = defaultdict(lambda: 0)
for line in lines:
    match line.split(" "):
        case ["$", "cd", "/"]:
            curdir = ["/"]
        case ["$", "cd", ".."]:
            curdir.pop()
        case ["$", "cd", d]:
            curdir.append(d)
        case ["$", "ls"]:
            pass
        case ["dir", name]:
            pass
        case [size, filename]:
            l = len(curdir)
            for i in range(l):
                path = '/'.join(curdir[:l - i])
                sizes[path] += int(size)
        case _:
            print("unknown line")

print(f"day07a: {sum([s for s in sizes.values() if s < 100000])}")
need_to_free = 30000000 - (70000000 - sizes['/'])
print(f"day07a: {min([s for s in sizes.values() if s > need_to_free])}")

# }}}
#%% day08 {{{

import numpy as np

def row_visibility(row):
    max = -1
    out = []
    for v in row:
        if v > max:
            max = v
            out.append(True)
        else:
            out.append(False)



def running_max(x1, axis=0):
    max = -np.ones(x1.shape[0])
    max = np.array([-1]*x1.shape[0])
    x1 = x1.copy()
    if axis == 0:
        x1 = x1.T
    for row in x1:
        new_max = max.copy()
        new_max[max < row] = row[max < row]
        row[:] = max[:]
        max = new_max
    if axis == 0:
        x1 = x1.T
    return x1


def scenic_score(x1, axis=0):

    if axis == 1:
        x1 = x1.T
    out = np.zeros_like(x1)
    for i, row in enumerate(x1):
        n = row.size
        for j in range(n):
            for k in range(j+1, n):
                out[i, j] += 1
                if row[j] <= row[k]:
                    break
    if axis == 1:
        out = out.T
    return out


with open("./input/day08.txt") as f:
    lines = f.read().splitlines()
    table = np.array([[int(v) for v in l] for l in lines])

top = table - running_max(table, axis=1)
bottom = table - np.flipud(running_max(np.flipud(table), axis=1))
left = table - running_max(table)
right = table - np.fliplr(running_max(np.fliplr(table)))

visible = np.any(np.stack([top,bottom,left,right], axis=2)>0, axis=2)

print(f"day08a: {np.count_nonzero(visible)}")

top =  scenic_score(table, axis=1)
bottom =  np.flipud(scenic_score(np.flipud(table), axis=1))
left = scenic_score(table)
right = np.fliplr(scenic_score(np.fliplr(table)))

score = np.prod(np.stack([top,bottom,left,right], axis=2), axis=2)
print(f"day08a: {np.max(score[:])}")

# }}}
#%% day08 better {{{

def visibility(arr):
    max = -1
    out = np.zeros_like(arr, dtype=bool)
    for i, v in enumerate(arr):
        if v > max:
            max = v
            out[i] = True
    return out


def scenic_score(arr):
    out = np.zeros_like(arr)
    for i, v0 in enumerate(arr):
        for v1 in arr[i+1:]:
            out[i] += 1
            if v0 <= v1:
                break
    return out


with open("./input/day08.txt") as f:
    lines = f.read().splitlines()
    table = np.array([[int(v) for v in l] for l in lines])


def apply(fn, table, dtype):
    out = [np.zeros(table.shape, dtype=dtype) for _ in range(4)]
    for i in range(table.shape[1]):
        out[0][:,i] = fn(table[:,i])
        out[1][:,i] = list(reversed(fn(list(reversed(table[:,i])))))
    for i in range(table.shape[0]):
        out[2][i,:] = fn(table[i,:])
        out[3][i,:] = list(reversed(fn(list(reversed(table[i,:])))))
    return out

visible = np.any(np.stack(apply(visibility, table, bool), axis=2), axis=2)
print(f"day08a: {np.count_nonzero(visible)}")
score = np.prod(np.stack(apply(scenic_score, table, int), axis=2), axis=2)
print(f"day08b: {np.max(score[:])}")

#%% day09 {{{

from copy import deepcopy

knots: list[tuple[int, int]] = [(0, 0) for _ in range(10)]

with open("./input/day09.txt") as f:
    lines = f.read().splitlines()

def sign(v):
    if v >= 0:
        return 1
    if v < 0:
        return -1

def update_tail(head, tail):
    dx = head[0] - tail[0]
    dy = head[1] - tail[1]

    if abs(dx) >= 2 and abs(dy) == 0:
        return (tail[0] + sign(dx), tail[1])
    if abs(dx) == 0 and abs(dy) >= 2:
        return (tail[0], tail[1] + sign(dy))
    if abs(dx) >= 2 or abs(dy) >= 2:
        return (tail[0] + sign(dx), tail[1] + sign(dy))

    return tail


history = [deepcopy(knots)]
for line in lines:
    d, n = line.split(" ")
    for i in range(int(n)):
        match d:
            case 'R': knots[0] = (knots[0][0] + 1, knots[0][1])
            case 'L': knots[0] = (knots[0][0] - 1, knots[0][1])
            case 'U': knots[0] = (knots[0][0]    , knots[0][1] + 1)
            case 'D': knots[0] = (knots[0][0]    , knots[0][1] - 1)
        for j in range(1, len(knots)):
            knots[j] = update_tail(knots[j-1], knots[j])
        history.append(deepcopy(knots))

print(f"day09a: {len(set([knots[1] for knots in history]))}")
print(f"day09b: {len(set([knots[-1] for knots in history]))}")


# }}}







