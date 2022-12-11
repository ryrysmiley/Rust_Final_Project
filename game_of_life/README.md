# Game of Life
The game of life is a cellular automaton. It is a zero player game, meaning that the game is played by a set of rules that are applied to a grid of cells. The rules are as follows:
1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by overpopulation.
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

# Instructions to run Game of Life
1. Install Rust using the instructions at https://www.rust-lang.org/tools/install 
2. Clone the repository
3. Run "cargo install rand" to install the rand crate
4. Run "cargo build"
5. Run "cargo run"
6. Enter the desired game mode
7. Enter the desired grid size

# Game Mode Descriptions
## Classic
The classic game mode is the original game of life.
## Mirror
The mirror game mode is the same as the classic game mode, except that the walls of the grid are mirrors. This means that cells on the edge of the grid still have 8 neighbors.
## Donut
The donut game mode is the same as the classic game mode, except that the walls of the grid are donuts. This means that cells on the edge of the are connected to cells on the opposite edge of the grid.

# Error Handling
## Invalid Game Mode
If the user enters an invalid game mode, the program will print an error message and exit.
## Invalid Grid Size
If the user enters an invalid grid size, the program will print an error message and exit.
