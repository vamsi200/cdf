from collections import Counter

host = input().strip()
guest = input().strip()
pile = input().strip()

some_letters = Counter(host) + Counter(guest)
no_idea_what_should_I_name_this_letters = Counter(pile)

if some_letters == no_idea_what_should_I_name_this_letters:
    print("YES")
else:
    print("NO")
