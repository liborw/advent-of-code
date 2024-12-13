import numpy as np
# Button A: X+94, Y+34
# Button B: X+22, Y+67
# Prize: X=8400, Y=5400

def gcd(a, b):
    while b:
        a, b = b, a % b
    return abs(a)

def lcm(a, b):
    if a == 0 or b == 0:
        return 0  # LCM is 0 if either number is 0
    return abs(a * b) // gcd(a, b)

#ba = np.array([94, 34])
#bb = np.array([22, 67])
#p = np.array([8400, 5400])
## p += 10000000
#p += 10000000000000

ba = np.array([26, 66])
bb = np.array([67, 21])
p = np.array([12784, 12176])
# p += 1000000
p += 10000000000000

a_est = min(p[0] // ba[0], p[1] // ba[1])

rem = p - ba * a_est

b_est = max(rem[0] // bb[0], rem[1] // bb[1])
rem = p - bb * b_est

print("stimate:",  a_est, b_est)

a_est = min(rem[0] // ba[0], rem[1] // ba[1])

print("stimate:",  a_est, b_est)

print(f"a: {ba}, a_est: {a_est}")
print(f"b: {bb}, b_est: {b_est}")
print(f"rem: {p - ba * a_est - bb * b_est } ")
#%%

n = 0
ix = []

for i in range(a_est):
    rem = p - ba * (a_est - i)
    nb = min(rem[0] // bb[0], rem[1] // bb[1])
    rem = rem - bb * nb

    if np.any(rem == 0):
        print(rem, i)
        ix.append((i, rem))
        n += 1

    if n >= 2:
        n = 0
        i0 = ix[0][0]
        i1 = ix[1][0]

        di = i1 - i0

        x0 = ix[0][1][0]
        x1 = ix[1][1][0]

        dx = x0 - x1

        print(rem[0] / dx)



    if nb < b_est:
        print("not found")
        break

    # print(f"a: {a_est - i}, b: {nb}, rem: {rem}")

    if np.all(rem == [0, 0]):
        print(f"Found a: {a_est - i} b: {nb}")
        break
else:
    print("not_found")

#%%

na = 80
nb = 40
p += 10000000000000




