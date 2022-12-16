
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

from math import prod
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
# }}}
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
#%% day12 {{{

import numpy as np

with open("./input/day12.txt") as f:
    lines = f.read().splitlines()

print('\n'.join(lines))

table = np.ones((len(lines), len(lines[0])), dtype=int)
start = None
goal = None
for i, l in enumerate(lines):
    for j, ch in enumerate(l):
        if ch == 'S':
            start = (i,j)
            ch = 'a'
        if ch == 'E':
            goal = (i,j)
            ch = 'z'
        table[i,j] = ord(ch) - ord('a')



def expand(table, s) -> list[tuple[int, int]]:
    i0, j0 = s
    cnds = []
    if i0 + 1 < table.shape[0]:
        cnds.append((i0 + 1, j0))
    if i0 > 0:
        cnds.append((i0 - 1, j0))
    if j0 + 1 < table.shape[1]:
        cnds.append((i0, j0 + 1))
    if j0 > 0:
        cnds.append((i0, j0 - 1))

    cnds = [(i, j) for i, j in cnds if table[i,j] - 1 <= table[i0,j0]]

    return cnds


def bfs(table, start, goal):
    visited = np.zeros_like(table, dtype=bool)
    visited[start[0], start[1]] = True
    queue = [start]
    parrent = dict()
    parrent[tuple(start)] = None
    while len(queue) > 0:
        node = queue.pop(0)
        if np.allclose(node, goal):
            i = 0
            while parrent[node] is not None:
                i += 1
                node = parrent[node]
            return i
        for n in expand(table, node):
            i,j = n
            if not visited[i,j]:
                queue.append(n)
                visited[i,j] = True
                parrent[tuple(n)] = tuple(node)
    return None



print(f"day12a: {bfs(table, start, goal)}")

starts = []
for i in range(table.shape[0]):
    for j in range(table.shape[1]):
        if table[i,j] == 0:
            starts.append((i,j))

print(f"day12b: {min(map(lambda s : bfs(table, s, goal), starts))}")




# }}}
#%% day13 {{{

import ast
from functools import cmp_to_key
from math import prod

with open('./input/day13.txt') as f:
    chunks = f.read().split("\n\n")

lines_by_2 = map(lambda l: l.splitlines(), chunks)


def compare(left, right) -> int:

    if isinstance(left, int) and isinstance(right, int):
        return left - right

    if isinstance(left, int) and isinstance(right, list):
        return compare([left], right)

    if isinstance(left, list) and isinstance(right, int):
        return compare(left, [right])

    for i in range(min(len(left), len(right))):
        cmp = compare(left[i], right[i])
        if cmp != 0:
            return cmp

    return len(left) - len(right)

result = []
packets = []
for left_str, right_str in lines_by_2:
    left = ast.literal_eval(left_str)
    right = ast.literal_eval(right_str)
    result.append(compare(left, right))
    packets.extend([left, right])


print(f"day13a: {sum([i+1 for i, v in enumerate(result) if v < 0])}")

dividers = [[[2]], [[6]]]
packets.extend(dividers)

packets = sorted(packets, key = cmp_to_key(compare))

for i, v in enumerate(packets):
    print(f"{i+1}: {v}")

print(f"day13a: {prod([i+1 for i, v in enumerate(packets) if v in dividers])}")

# }}}
#%% day14 {{{

import re

# parse input
tokens = re.compile(r"(\d+),(\d+)")

with open("./input/day14.txt") as f:
    rocks = []
    for line in f.read().splitlines():
        rock = []
        for token in  tokens.finditer(line):
            rock.append((int(token[1]), int(token[2])))
        rocks.append(rock)


def norm(x):
    if x < 0:
        return -1
    elif x > 0:
        return 1
    else:
        return 0

# interpolate
sparse_map = dict()
y_max = 0
for rock in rocks:
    for i in range(1, len(rock)):
        x0, y0 = rock[i-1]
        x1, y1 = rock[i]
        xi, yi = x0, y0

        if y_max < max(y0, y1):
            y_max = max(y0, y1)

        while abs(xi - x1) > 0 or abs(yi - y1) > 0:
            sparse_map[(xi, yi)] = '#'
            xi = xi + norm(x1 - xi)
            yi = yi + norm(y1 - yi)
            print(xi, yi, x1, y1, x1 - xi)

        sparse_map[(x1, y1)] = '#'


def boundin_box(smap:dict[tuple, str])-> tuple:
    bb = [float('inf')]*2 + [float('-inf')]*2
    for x, y in smap.keys():
        if bb[0] > x: bb[0] = x
        if bb[1] > y: bb[1] = y
        if bb[2] < x: bb[2] = x
        if bb[3] < y: bb[3] = y
    return tuple(bb)


def print_sparse_map(smap:dict[tuple, str]):
    bb = boundin_box(smap)
    for y in range(bb[1]-1, bb[3]+2):
        for x in range(bb[0]-1, bb[2]+2):
            print(smap.get((x,y), '.'), end='')
        print()


def add_sand_grain(smap: dict[tuple,str], y_max: int, x:int) -> bool:
    y = 0
    while (y <= y_max):
        if (x,y + 1) not in smap:
            y += 1
        elif (x - 1,y + 1) not in smap:
            x -= 1
            y += 1
        elif (x + 1,y + 1) not in smap:
            x += 1
            y += 1
        else:
            smap[(x,y)] = 'o'
            return True

    smap[(x,y)] = 'o'
    return False

step = 0
while add_sand_grain(sparse_map, y_max, 500):
    step += 1

print_sparse_map(sparse_map)
print(f"day14a: {step}")

while (500, 0) not in sparse_map:
    add_sand_grain(sparse_map, y_max, 500)
    step += 1

print_sparse_map(sparse_map)
print(f"day14b: {step}")

# }}}
#%% day15 {{{

import re
import numpy as np
import itertools

tokens = re.compile(r"(-?\d+)")

with open("./input/day15.txt") as f:
    sensors_and_beacons = []
    for line in f.read().splitlines():
        sensors_and_beacons.append([int(m[1]) for m in tokens.finditer(line)])

line = 2000000

def neigbourhood(x0,y0,r,yi):
    dy = abs(yi - y0)
    dx = r - dy
    for xi in range(x0 - dx, x0 + dx + 1):
        if abs(xi - x0) + abs(yi - y0) <= r:
            yield (xi, yi)

sparse_map = dict()
for sx,sy,bx,by in sensors_and_beacons:
    sparse_map[(sx,sy)] = 'S'
    sparse_map[(bx,by)] = 'B'

    r = abs(sx - bx) + abs(sy - by)
    for xi, yi in neigbourhood(sx, sy, r, line):
        if (xi, yi) not in sparse_map:
            sparse_map[(xi,yi)] = '#'



def boundin_box(smap:dict[tuple, str])-> tuple:
    bb = [float('inf')]*2 + [float('-inf')]*2
    for x, y in smap.keys():
        if bb[0] > x: bb[0] = x
        if bb[1] > y: bb[1] = y
        if bb[2] < x: bb[2] = x
        if bb[3] < y: bb[3] = y
    return tuple(bb)

def print_sparse_map(smap:dict[tuple, str], margin=0):
    bb = boundin_box(smap)
    for y in range(bb[1]-margin, bb[3]+margin+1):
        for x in range(bb[0]-margin, bb[2]+margin+1):
            print(smap.get((x,y), '.'), end='')
        print()

bb = boundin_box(sparse_map)
print(f"day15a: {sum([1 for x in range(bb[0], bb[2]+1) if sparse_map.get((x,line),'.') in ['#', 'S']])}")



#%% }}}
#%% day16

import re
import numpy as np
import itertools
from dataclasses import dataclass

class NameMap(object):

    def __init__(self, init_names=[]):
        self.i = 0
        self.names = dict()
        for n in init_names:
            self[n]

    def __getitem__(self, key):
        if key not in self.names:
            self.names[key] = self.i
            self.i += 1
        return self.names[key]

    def size(self):
        return len(self.names)


@dataclass
class Valve:
    id: int
    name: int
    flow: int
    next: list[int]

nmap = NameMap(['AA'])
valves = []
tokens = re.compile(r"([A-Z]{2}|\d+)")
with open("./input/day16_test.txt") as f:
    for line in f.read().splitlines():
        name, vflow, *to = tokens.findall(line)
        valves.append(Valve(nmap[name], name, int(vflow), [nmap[n] for n in to]))
valves = sorted(valves, key = lambda v: v.name)
N = len(valves)

dist = np.ones([nmap.size()]*2, dtype=int)*10_000
for v in valves:
    dist[v.id,v.id] = 0
    for nv in v.next:
        dist[v.id, nv] = 1
for k in range(N):
    for i in range(N):
        for j in range(N):
            if dist[i,j] > dist[i,k] + dist[k,j]:
               dist[i,j] = dist[i,k] + dist[k,j]


def dfs(v: int, ttl: int, state: list, flow: int) -> int:

    if ttl < 0:
        return 0, [v], state.copy()

    if ttl == 0:
        return flow, [v], state.copy()

    curr_flow = 0
    for valve in valves:
        if state[valve.id] == 1:
            curr_flow += valve.flow

    max_flow = flow + curr_flow * ttl
    hist = []
    nstate = state.copy()
    for valve in valves:
        if valve.flow > 0 and state[valve.id] == 0:
            d = dist[v, valve.id] + 1
            ns = state.copy()
            ns[valve.id] = 1
            nf = flow + d * curr_flow
            f, nhist, ns = dfs(valve.id, ttl - d, ns, nf)
            if max_flow < f:
                max_flow = f
                hist = nhist
                nstate = ns.copy()

    return max_flow, [v] + hist, nstate.copy()


flow, path, state = dfs(0, 24*2, [0]*len(valves), 0)

print(f"day16a: {flow/2}")
print(path, state)





#%%






