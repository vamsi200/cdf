a = []
x1,x2,x3 = map(int, input().split())
a.append(x1)
a.append(x2)
a.append(x3)

a = sorted(a)

print(a[1] - a[0] + a[2] - a[1])
