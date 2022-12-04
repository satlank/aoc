# Advent of Code Solutions

Solutions for [Advent of Code](https://adventofcode.com/) puzzles are
organised by language, then year, then day.  Each day is runnable on its
own and does not depend on solutions for other days.

## Rust

Using this as an opportunity to get more familiar with Rust, so not
necessarily (well, unlikely) very clean code.

Generating fresh rust project for each day by

    cargo new --bin --vcs none --edition 2018 d01

## Locking

The input files are encrypted, but can be kept locally unencrypted as that is what the programs expect.  To encrypt run

    ./lock.bash <GPG RECIPIENT>
