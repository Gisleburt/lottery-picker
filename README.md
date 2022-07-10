Euromillions Lottery Picker
===========================

Randomly picks lottery numbers for the Euromillions

Created to help my partner on their programming journey.

Usage:
------

### Lucky dip

Just gives you some random numbers:

```shell
$ cargo run --bin lucky-dip
   Compiling lottery-picker v0.1.0 (/Users/danielmason/projects/lottery-picker)
    Finished dev [unoptimized + debuginfo] target(s) in 4.68s
     Running `target/debug/lucky-dip`
Numbers: [2, 12, 21, 34, 52]
Bonus: [2, 7]
```

### From pool

Lets you provide a list of numbers that have recently won (this doesn't improve your chances at all but
what the hey 😅):

```shell
$ cargo run --bin from-pool -- --numbers 1,2,3,4,5,6,7,8,9,10 --lucky 1,2,3
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/from-pool --numbers 1,2,3,4,5,6,7,8,9,10 --lucky 1,2,3`
Numbers: [3, 6, 7, 8, 10]
Bonus: [1, 2]
```

### From Recent Wins

Uses a pool of only numbers that appeared in the last six months.

```shell
$ cargo run --bin from-recent-wins
    Finished dev [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/from-recent-wins`
Numbers: [1, 6, 7, 20, 48]
Bonus: [7, 11]
```

By default, `from-recent-wins` is biased by how many times a given number appeared over the last 6 months, if you don't
want this bias, use `--dedupe`

```shell
cargo run --bin from-recent-wins -- --dedupe
    Finished dev [unoptimized + debuginfo] target(s) in 0.81s
     Running `target/debug/from-recent-wins --dedupe`
Numbers: [8, 31, 35, 43, 44]
Bonus: [6, 8]
```

Note 1: It would be best practice to gate dependencies with features, but it makes running the examples more work and
this is just supposed to be a small teaching example.

Note 2: If you happen to use this yourself and win, I'm happy for you, you _**do not**_ owe me anything, its _literally_
random, I am not magic 😅.
