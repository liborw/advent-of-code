import sys

filename = sys.argv[1]


class Car:
    def __init__(self, name, tempo: int, length: int, position: int, lane: int) -> None:
        self.name = name
        self.tempo = tempo
        self.length = length
        self.position = position
        self.lane = lane

    def __str__(self) -> str:
        return f"Car({self.position}, {self.lane})"

    def step(self, cars: list["Car"], lanes: int) -> bool:  # vykona jeden krok auta
        if self.car_ahead(cars):
            if self.lane + 1 < lanes and self.can_switch(cars, self.lane + 1):
                self.lane += 1
                return True
            else:
                return False

        else:
            self.position += 1
            if self.lane > 0 and self.can_switch(cars, self.lane - 1):
                self.lane -= 1

            return True

    def car_ahead(self, cars: list["Car"]) -> bool:  # je nejake auto pred timto autem
        for car in cars:
            if car is self:
                continue

            if car.lane != self.lane:
                continue

            if car.position - car.length == self.position:
                return True

        return False

    def can_switch(
        self, cars: list["Car"], lane: int
    ) -> bool:  # je mozne se zaradit do pruhu
        for car in cars:
            if car is self:
                continue

            if car.lane != lane:
                continue

            if self.position - self.length + 1 <= car.position <= self.position:
                return False

            if car.position - car.length + 1 <= self.position <= car.position:
                return False

        return True

NAMES = "123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
def load_data(filename: str) -> tuple[int, int, list[Car]]:
    cars = []
    with open(filename) as f:
        distance = int(f.readline().strip())
        lanes = int(f.readline().strip())
        i = 0
        for line in f:
            cars.append(
                Car(NAMES[i], *map(int, line.strip().split()))
            )  # omlouvam se za tento zapis :D
            i += 1

    return distance, lanes, cars


def step(cars: list[Car], lanes: int, tick: int) -> bool:
    changed = False
    for car in cars:
        if tick % car.tempo == 0:
            if car.step(cars, lanes):
                changed = True

    return changed


def draw(distance, lanes, cars):
    l = distance + 5

    img = [['.']*l for _ in range(lanes)]
    for i in range(lanes):
        img[i][distance] = '|'

    for car in cars:
        for i in range(car.position - car.length + 1, car.position + 1):
            if 0 <= i < l:
                img[car.lane][i] = car.name

    for line in reversed(img):
        print(''.join(line))




distance, lanes, cars = load_data(filename)

first = None  # kdy prvni auto dosahne cile

print("init:")
draw(distance, lanes, cars)
i = 0
while True:
    if step(cars, lanes, i):
        print(f"time: {i}")
        draw(distance, lanes, cars)

    for car in cars:
        if car.position >= distance and not first:
            first = i

    if all(car.position >= distance for car in cars):
        break

    i += 1

assert first is not None
print("First:", first, "Last:", i, "Diff:", i - first)
