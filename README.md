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


## Examples

|    board           | number of moves | solution(s) 					|
|--------------------|-----------------|-------------------------------------------------|
| 1,2,3,4,0,5,7,8,6 |	2	      |	RD 						|
| 1,2,3,7,4,5,0,8,6 |	4	      |	URRD						|
| 1,2,3,4,8,0,7,6,5 |	5	      |	DLURD 						|
| 4,1,3,7,2,6,5,8,0 |	8	      |	LLUURDDR 					|
| 1,6,2,5,3,0,4,7,8 |	9	      |	LURDLLDRR 					|
| 5,1,2,6,3,0,4,7,8 |	11	      |	LLURRDLLDRR 					|
| 1,2,6,3,5,0,4,7,8 |	13	      |	ULDLDRRULURDD 					|
| 3,5,6,1,4,8,0,7,2 |	16	      |	RRUULLDRDRUULDRD 				|
| 4,3,6,8,7,1,0,5,2 |	18	      |	URRULDDRULDLUURDRD 				|
| 3,0,2,6,5,1,4,7,8 |	21	      |	DRULDLURRDLLDRRULURDD or DLURDRULDLURDRULDLDRR  |
| 0,1,2,3,4,5,6,7,8 |	22	      |	RDLDRRULLDRUURDDLLURRD or DRRULLDDRUURDLLURRDLDR|
| 5,0,3,2,8,4,6,7,1 |	23	      |	LDDRRULLDRRULLDRUULDDRR 			|
| 8,7,4,3,2,0,6,5,1 |	25	      |	DLULURDRULDDLUURDRDLLURRD 			|
| 8,7,6,5,4,3,0,2,1 |	28	      |	UURDRDLLUURDRULDDRULDLUURDRD or UURDLDRURDLLUURDRULDDLUURDDR |
| 8,7,6,5,4,3,2,1,0 |   30	      |	ULLURDDRUULDDLUURDDRUULDDLURRD or ULULDDRUULDDRUURDDLUURDLULDRDR |

source:
https://www.andrew.cmu.edu/course/15-121/labs/HW-7%20Slide%20Puzzle/lab.html
