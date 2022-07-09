Euromillions Lottery Picker
===========================

Randomly picks lottery numbers for the Euromillions

Created to help my partner learn some programming.

Usage:
------

Lucky dip, ust gives you some random numbers:

```shell
$ cargo run --bin lucky-dip
   Compiling lottery-picker v0.1.0 (/Users/danielmason/projects/lottery-picker)
    Finished dev [unoptimized + debuginfo] target(s) in 4.68s
     Running `target/debug/lucky-dip`
Numbers: [2, 12, 21, 34, 52]
Bonus: [2, 7]
```

Recent numbers, lets you provide a list of numbers that have recently won (this doesn't improve your chances at all but
what the hey 😅):

```shell
$ cargo run --bin recent-numbers -- --numbers 1,2,3,4,5,6,7,8,9,10 --lucky 1,2,3
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/recent-numbers --numbers 1,2,3,4,5,6,7,8,9,10 --lucky 1,2,3`
Numbers: [3, 6, 7, 8, 10]
Bonus: [1, 2]
```

Note: If you happen to use this yourself and win, I'm happy for you, you _**do not**_ owe me anything, its _literally_
random 😅.
