# 2022 Day 1

Honestly, as with each year.. I like to use this opportunity, when I find some free time, to learn a little more Rust.

## Part 1

Going to do this super ugly, but I've started mentally getting "better" at visualizing the borrowing of memory and data. While I'm going to write this quickly and without a lot of thought re: efficiency, I realize that if you're just going to read stuff without modifying it, you can do some borrowing.

So my mental model going in is:

- Read from text file into an input var
- Borrow the input var and come up with a new version of that data for each elf
- Borrow that data and process it to make new values for each meal's calories
- Put it all into a vector and then find the largest <-- inefficient, but I bets there's a twist for part two

## Part 2

There's always a twist with these puzzles, and I roughly knew where it might live.

Using the code form part1, slight tweak using the result vector to sort and slice the values.

Again, near-zero focus on optimization, just doing the work and learning a little bit again on how to do it. And even which extensions to install in my editor (VS Code)

## Thoughts

- Could have avoided using an output vector probably, but this was simple and ran fast enough
- Mentally thinking about what's modified vs clonable vs borrowable is a neat exercise
