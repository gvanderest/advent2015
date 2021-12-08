# 2021 Day 3

## Part 1

This took a little bit to understand and really perform-- and I likely did it super inefficiently, but iterating over each string and comparing the values in the positions worked fairly well. It was not insane, and the parsing of the string into a number was a quick Google away.

## Part 2

This took me a while. I made a mistake early on and didn't sieve the list at all.. do I was constantly comparing against the full list. Once I figured out that was my mistake, I pulled out a `HashSet` and got the solution fairly quickly. Didn't attempt to optimize at all, but have some ideas of how I'd have split up the functions or stored values to reduce iterating.

## Thoughts

-   Spun my wheels a lot because of a mistake re: removing items from the list.
-   HashSet is nice for storing/tracking items.
-   Getting a bit better of a grasp of when to borrow or clone, not even remotely good at it yet though.
-   Wondering if I'm using `match` incorrectly sometimes to be able to do more complex expressions, like `match x` and then using boolean logic for paths? Maybe I'm confusing it with Elixir or Kotlin or something else.
