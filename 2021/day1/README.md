# 2021 Day 1

## Part 1

At first glance, this was harder than some of the previous years' first day problems from the standpoint of language knowledge, but that's good too! Struggled a bit with remembering some of the base commands from previous work: reading from a file, parsing it into integers, and doing some logic over it. Part one was not super difficult, and starting with tests is definitely helping me.

## Part 2

Having a sliding window meant I'd need to do more than just iterate over values, and instead do a little storing of multiple values and math. Had to learn how to use vectors and convert the sliding window values to a separate vector, which I later used in a similar fashion to the first part.

## Thoughts

-   I wonder if there's a way more functional programming approach I could have used with moving indexes and reducers to collect? I'll probably find out way later.
-   With my primary goal of just being "solve the problem", I did see where I could have part two reuse the logic from part one.. but I didn't feel like refactoring the logic to make that work.
-   I wish my VSCode gave me better hinting at methods I can use.. having to guess when I can do `.collect()` or `.unwrap()` is a little annoying.
