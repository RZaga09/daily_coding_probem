'''
Given a list of integers, write a function that returns the largest sum of non-adjacent numbers. 
Numbers can be 0 or negative.
For example, [2, 4, 6, 2, 5] should return 13, since we pick 2, 6, and 5.
[5, 1, 1, 5] should return 10, since we pick 5 and 5.
'''

test1 = [2, 3, 6, 2, 5]
if len(test1) == 5:
	ans = max(test1[0] + test1[2] + test1[4], test1[1] + test1[3], test1[1] + test1[4], test1[0] + test1[3])

print(test1)
print(ans)

test2 = [5, 1, 1, 5]
if len(test2) == 4:
	ans = max(test2[0] + test2[2], test2[0] + test2[3], test2[1] + test2[3])

print(test2)
print(ans)