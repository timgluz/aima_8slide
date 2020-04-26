# AIMA 8Slide Puzzle

A small solver for 8Slide puzzle written in Rust;
The search algorithms are adopted from the book "AI: Modern Approach 3rd edition";

## Usage:

```
# if using cargo
cargo run -- 1,2,3,4,5,0,7,8,6
cargo run -- 1,2,3,4,5,0,7,8,6 uniform_cost

# is using compiled binary
./aima_8slide 1,2,3,4,5,0,7,8,6
```

## Supported Algorithms

#### Uninformed Search

* DepthFirst
* BreadthFirst
* UniformCost


