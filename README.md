# Sudoku Solver: Wave Function Collapse
The problem is seemingly simple: given a sudoku board, how can that board be solved? There are 
many ways to solve the problem, but the one untilized here is the wave function collapse algorithm. This program
is a command line executable written in Rust that can take an input board (in the format of a CSV) and output a new file containing the solved board.

## Wave function collapse
The wave function collapse algorithm is an algorithm that has an intimidating name, but is quite tame from a programming perspective.
The fundimental idea is a generation algorithm, which assumes that any unknown is just a combination of all the possible states it could be, at once.
For example, if you have a sudoku row of: [5, 3, 9, 2, x, 1, 4, y, 6], where x and y are tiles that you don't know, the wave function collapse algorithm 
would say that instead of x and y simply be unknown, or having no value, x and y are considered to both be at the state 7 and 8 at once. Then if (for example)
[4, y, 6] are in a square that already has 8 in it, the y's state is just 7, and it "collapses" into a single state of 7. Once y is at a single state, the 
origional list is now [5, 3, 9, 2, x, 1, 4, 7, 6], so x only has one possible state: 8, meaning that the entire row collapses into a solution: 
[5, 3, 9, 2, 8, 1, 4, 7, 6], and is solved.

## How to use
Using the cargo build too, clone the repository, the run `cargo build --release`, which will output the compiled executable into `target/release/sudoku_solve`.
To run it, simply put `[path to executable] [path to board] [additional arguements]`. For example: `./sudoku_solve board.csv -v -a`. To view all arguements, 
run `[path to executable] -h`.

## Limitations
At the moment, due to the nature of algorithm in use, there are some limitations to what can be solved by the solver. For example, if, after all possible iterations,
there are still tiles that cannot collapse, then there isn't anything that the algorithm can do. In this situation, the solver will spit out the unfinished 
board, and human intervention will be required.

## Goals
- Command line input
- Take input files, output files
- Solving the input using wave function collapse

## Stretch Goals
- GUI
- Output a nice visualization
