
#%% day01

with open('./input/day01.txt', 'r') as f:
    elves = [0]
    for line in f.readlines():
        try:
            elves[-1] += int(line[:-1])
        except ValueError:
            elves.append(0)
print(f"day01a: {max(elves)}")
print(f"day01b: {max(sorted(elves)[-3:])}")


#%% day03

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





