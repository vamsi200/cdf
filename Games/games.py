n = int(input())
teams = []
count = 0
for _ in range(n):
    h, a = map(int, input().split())
    teams.append((h, a))
for i in range(n):
    for j in range(n):
        if i != j: 
            if teams[i][0] == teams[j][1]:
                count += 1

print(count)
