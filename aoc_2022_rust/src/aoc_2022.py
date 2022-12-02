
with open('./aoc_2022_rust/input/day01.txt', 'r') as f:
    elves = [0]
    for line in f.readlines():
        try:
            elves[-1] += int(line[:-1])
        except ValueError:
            elves.append(0)
print(f"day01a: {max(elves)}")
print(f"day01b: {max(sorted(elves)[-3:])}")


