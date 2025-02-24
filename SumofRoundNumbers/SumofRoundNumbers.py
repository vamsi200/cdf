t = int(input())

for i in range(t):
    k = int(input())
    parts = []
    factor = 1

    while k > 0:
        digit = k % 10
        if digit > 0:
            parts.append(digit * factor)
        k//=10
        factor*=10

    print(len(parts))
    print(*parts)
 
