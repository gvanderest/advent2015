# 2015 - Day 5

## Part 1

Started to move into indexing strings and jumping around them, which required using some slices. After breaking down the rules into comments and then writing the logic for each rule, it was not too difficult to implement.

## Part 2

Got stuck on a typo where I forgot to jump forward far enough for checking the patterns didn't overlap. Once I wrote out an extra test case for this scenario, I was able to figure out what I did wrong.

## Thoughts

-   Writing out the tests at the start to prove out the logic using examples from the wording is very helpful.
    -   Can still screw up as a result though, because some examples might be missing or obscured.
-   I'm not entirely sure how to properly define constants outside of a function.. especially if it's a weird type like a vector.
