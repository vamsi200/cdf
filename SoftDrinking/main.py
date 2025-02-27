n, k, l, c, d, p, nl, np = map(int, input().split())

drinks = k*l//nl
limes = (c*d)
salt = p//np

mxt = min(drinks//n,limes//n,salt//n)
print(mxt)

