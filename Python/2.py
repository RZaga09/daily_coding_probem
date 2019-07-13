'''
Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.
For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].
'''

from random import randint

arr = []
new = []
for i in range(0, randint(2, 5)):  # decides length of list (between 2-5)
    arr.append(randint(1, 50))  # appends values to list

j = arr[0] * arr[1]

for i in range(2, len(arr)):
	j = j * arr[i]

for i in range(0, len(arr)):
	new.append(j)
	new[i] = int(j / arr[i])

print(arr)
print(new)