n = int(input())
l = list(map(int, input().split())) 

police = 0
count = 0

for e in l:
    if e == -1:
        if police > 0:
            police-=1
        else:
            count+=1
    else:
        police+=e

print(count)
