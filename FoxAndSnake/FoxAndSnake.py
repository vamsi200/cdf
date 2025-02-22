n, m = map(int , input().split())

for i in range(n):
    if i % 4 == 0:
        print('#'*m)
    elif i % 4 == 2:
        print('#' + '.' * (m-1))
    else:
        print('.'*(m-1) + '#')

