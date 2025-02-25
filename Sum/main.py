t = int(input())

data = []
for i in range(t):
    l = list(map(int, input().split()))
    data.append(l)

for d in data:
    x,y,z = d
    if x + y == z or y + z == x or x + z == y:
        print("YES")
    else:
        print("NO")

