# 2022 Day 3 - Rucksack Reorganization

## Part 1

Initial thoughts..

- Read the lines in
- Split the lines in half
- Store letters->counts mapping in hashmap
- The instant we encounter a 1 for any item when we re-check, return the letter
- Convert letter to a score
- Sum up total scores

## Part 2

So instead, we have groups of three lines, which correspond to a group of elves..

- Read 3 lines at a time
- Use a hashmap that tracks counter-per-bag across all three lines
- If on the third bag we find a 2, then we kick out and return letter
- SAME: Convert letter to score
- SAME: Sum up total scores

## Thoughts

Part one was not bad, definitely didn't think of optimizing or making the code cleaner. Going through the list twice (once for each half) didn't hurt too much.

Part two took a little thinking, because I was initially only doing a hashmap of counts, but forgot that you could have multiple of the same item in a rucksack.

After adding a separate hashmap for tracking seen items in that rucksack, I was able to prevent multiple counts per rucksack which made it easy.

Bug that took some time: As part of my chunking algorithm, I totally forgot to clear the chunk list after getting 3 sacks together.. and had a little "what the heck" moment wondering why it wasn't firing multiple times. Added the list clearing and chuckled.

Could probably optimize part two by using slices or something instead of building a vector to pass down.. but this was a little simpler to grok in my head initially.
