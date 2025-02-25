n,k = map(int, input().split())

total = 0
solved = 0
available = 240 -k
for i in range(1, n+1):
    t_to_solve = 5 * i
    total+= t_to_solve
    
    if total <= available:
        solved+=1
    else:
        break

print(solved)
