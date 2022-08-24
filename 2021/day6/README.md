# Day 6

- Initial laternfish are created once every 7 days, but unsynchronized between existing population
- Each fish has a number of days until it creates another one
- New lanternfish have a starting age of 8
- 0 is a valid date

## Part 1

Thinking I'll just have an array of numbers, and for each day.. pass over the list, outputting a new array. Reminds me of a pure function implementation of `X -> f(x) -> Y` but might run into performance issues later on, becaues of all the looping.

- Used a flat_map to allow returning (1) a reset fish with a new fish, or (2) the ticked down fish

## Part 2

Just changing the number of days seems like a simple fix, but the expected inefficiency showed its head.

I'm going to just let it run to get an answer, but have a thought of optimization: What if instead of doing passes over the Vector, I perform all the computation for a single fish, then pass the result down the line to be computed for the next fish? I don't even know the complexity it's coming from and moving towards, but it would be an interesting solve if I have time to implement it.

Something like.. for each initial fish, calculate its resets and how many offspring it will have from this point to the end, including the "remaining days available" to compute those values down the line.

After sleeping on it, I thought about applying a HashMap or array to store all of the counts of fish at that day position. As each day ticks..

- pop the value off the front to determine how many fish births will occur
- increment reset position by birth_count, for fish resetting
- increment new fish position by birth_count for babies
- add a zero to the end

I picked a HashMap just because it was slightly easier for me to visualize and update values in my mind, but an array would likely be much faster
