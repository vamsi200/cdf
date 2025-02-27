n = int(input())
l = list(map(int, input().split()))

count = 0
best = l[0]
worst = l[0]

for i in range(1, n):
    if l[i] > best:
        count+=1
        best = l[i]
    elif l[i] < worst:
        count+=1
        worst = l[i]
print(count)
