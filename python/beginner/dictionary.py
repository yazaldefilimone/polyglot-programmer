me = {
    "name": "Yazalde",
    "last_name": "Filimone",
    "nationality": "Mozambican"
}

print('----dictionary----')

print(me)
print(me['name'])
print(me['last_name'])

print("My name is %s" % (me["name"]))  # My name is Yazalde
print("But you can call me %s" %
      (me['last_name']))  # But you can call me Filimone
print("And by the way I'm %s" % (me["nationality"]))

print('----for...in----')
bookshelf = [
    "The Effective Engineer",
    "The 4 hours work week",
    "Zero to One",
    "Lean Startup",
    "Hooked"
]

for book in bookshelf:
    print(book)

print('----for...in..dictonary----')

for key in me:
    print(f'the key:{key} and value: {me[key]}')
print('----for...in..dictonary...items----')

for key, value in me.items():
    print(f'the key:{key} and value: {value}')
