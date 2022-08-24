# Day 7

## Part 1

Try iterating over each number, and each position in the range of smallest number to largest number.. calculate distance for all crabs and figure out the best spot to use.

Initial thoughts:

- This seems inefficient, because it's O(N^2) (maybe?) for each crab against each crab.. kinda
- With more crabs, it might make sense to count how many crabs are in a spot, similar to day6.. and start from there, moving outwards? It would seem like the spot which is most efficient will be nearest the highest concentration of crabs
- Part two is likely going to involve X+Y coordinates, so this approach will get even less efficient

## Part 2

I was wrong, function for computing distance changed slightly and re-ran. Worked! Inefficient, but it worked!
