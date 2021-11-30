# Day 3

## Part 1

Not insane, but started to learn how to use the `HashMap` collection, which comes with some quirks regarding borrowing. I'm struggling a bit with the borrow-checker, and understanding when something is a mutable borrow, a reference, or just passed. I tried to split some logic into a separate function for incrementing values at coordinates, and encountered all sorts of problems-- I'll revisit this later.

## Part 2

This was pretty easy, but obviously came with a copy/paste of the existing logic. Made some use of `match` a little more for deciding which coordinates to use and how to copy them back.

## Things to Dig Into

-   Learning how to "lend out" via reference or mutability the HashMap and coordinates to a separate function to do the hashmap update
-   I'm probably making a lot of unnecessary copies/inefficiencies
-   I've read in the past about how Rust supports a lot of functional programming paradigms. Approaching this using map/foreach/reduce might be a better direction in the future?
