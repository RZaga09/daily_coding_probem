'''
Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
'''

from random import randint

h = False
k = randint(1, 50)  # random value between 1-50
arr = []
for i in range(0, randint(2, 5)):  # decides length of list (between 2-5)
    arr.append(randint(1, 25))  # appends values to list

for i in range(len(arr)):
    for j in range(len(arr)):
        if arr[i] + arr[j] == k and i != j:
            h = True
            break

print(k)
print(arr)
print(h)