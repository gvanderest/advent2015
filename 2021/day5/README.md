# 2021 Day 5

## Part 1

Initial thinking is that this puzzle might be easier than the previous at first glance. I'll probably be proven wrong in part two.

I am going to attempt to solve this by:

-   ~~Iterating over the list of inputs and figuring out the max X/Y coordinates (inefficient? but simpler)~~
-   ~~Create a two-dimensional vector of coordinates, representing 2D coordinates~~
-   ~~Initialize all values with zero~~
    -   Didn't need to do any of the above, because instead of a two-dimensional vector of zeroes, I used a HashMap with the key being coordinates
-   Iterate over all inputs..
    -   Filter out diagonals
    -   Follow the range of X1->X2 and Y1->Y2 and increment all spots by 1
-   Iterate over the vectors and count the spots with a value >= 2

## Part 2

I got a little stuck here because of needing to figure out how to move up/down using unsigned integers and learn a bit about how to do conversions between unsigned/signed integers. I also got stuck in my approach of using min/max X and Y values, which ultimately led to me always increasing X. In the end, rewriting the loop to calculate both X/Y values the same way (supporting increase/reduction) was the fix.

## Thoughts

-   I am not great at cartesian math in my head.
-   I did a lot more functional programming this time, and it was super confusing initially until I got the `rust-analyzer` plugin in VS Code working with type annotations for arguments/outputs of calls.
-   Not crazy, but my brain got stuck on a few concepts.
-   I am purposefully not trying to optimize my previous code to support parts 1 and 2-- which could have probably been achieved with a simple "process_diagonals" flag, but it still makes my brain a little sad each time I don't. :-P
