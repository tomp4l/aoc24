#!/bin/bash

cargo build --release

time for i in $(seq 1 25); do
    echo day $i:
    ./target/release/aoc24 $i
done
