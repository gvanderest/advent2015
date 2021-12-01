# Day 4

## Part 1

Learning how to use my first crate with md5, as well as how to interact a bit with more complex types like the `md5::Digest`.  The logic itself was kind of simple, but I had a heck of a time trying to find hashes that start with a series of zeroes.. thinking I maybe needed to do something bitwise.  Eventually landed on using `format!("{:?}", digest)` but don't feel like that's super clean.

## Part 2

Same logic as 1, just a longer comparison string.

## To Look Into
- Maybe because my loop just increments forever, might be a smarter way of saying like.. `for x in 1..INFINITY { ... }`
- Not quite sure how to get the Digest turned into a nice hash without using `format!()` at the moment, there's probably something simple
