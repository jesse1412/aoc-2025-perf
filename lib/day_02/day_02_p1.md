# Step 1 - Algo optimisations
We can solve this mathematically. Consider the range 1523 - 16,810,256.
1523 - 16,810,256
Start with the range 1523 - 9999.
We get the first repeating number, 16 (1616), and the max, 99 (9999).
Now just count how many repeating numbers are in the range (1+99-16).

Next do the next level, 10,000 - 99,999.
There are no repeat numbers as the digit count is odd.

Next do the middle level:
100,000 - 999,999
Same method as before, but 3 digits now:
(999 - 100) + 1

Skip the next level as odd digits.

Next do final level:
10,000,000 - 16,810,256
1,000 to 1680
(1680 - 1000) + 1

# Step 2, additional optimisations.
Not needed at this time, already 5 orders of magnitude faster (71306137ns -> 780ns)