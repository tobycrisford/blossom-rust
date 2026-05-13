# Solving Blossom with Rust

This was an attempt to learn some of the basics of Rust by building a solver for the game Blossom (make as many words as you can
from a set of letters, where each letter can be used an unlimited number of times but one of the letters is mandatory).

The solver works by storing the dictionary in a tree data structure. I then compare the performance to a simple baseline that just checks
every word in the dictionary in a loop.

This was a good way to learn about ownership and borrowing and lifetimes!
