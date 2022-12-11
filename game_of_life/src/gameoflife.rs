/*Game of Life Grid*/
extern crate rand;
use::std::io;
use std::fs::File;
use std::io::Write;
pub struct GoLGrid {
    grid: Vec<Vec<char>>,
    grid2: Vec<Vec<char>>,
    grid3: Vec<Vec<char>>,
    grid4: Vec<Vec<char>>,
    grid5: Vec<Vec<char>>,
    gen_num: usize,
    width: usize,
    height: usize,
}

impl GoLGrid {
    //create a new grid
    pub fn new(width: usize, height: usize) -> GoLGrid {
        let mut grid = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push('-');
            }
            grid.push(row);
        }
        GoLGrid {
            grid: grid,
            grid2: Vec::new(),
            grid3: Vec::new(),
            grid4: Vec::new(),
            grid5: Vec::new(),
            gen_num: 1,
            width: width,
            height: height,
        }
    }
    //get gen num
    pub fn get_gen_num(&self) -> usize {
        return self.gen_num;
    }
    //return grid as string
    pub fn return_grid(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.grid[y][x]);
            }
            output.push('\n');
        }
        output.push_str(&format!("Generation: {}", self.gen_num));
        output.push('\n');
        output
    }
    //randomly populate grid
    pub fn randomize(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if rand::random::<f32>() < 0.5 {
                    self.grid[y][x] = 'X';
                }
            }
        }
    }
    //check if grid matches other grids
    pub fn check_match(&self) -> bool {
        if self.grid == self.grid2 && self.gen_num > 1{
            return true;
        } else if self.grid == self.grid3 && self.gen_num > 2{
            return true;
        } else if self.grid == self.grid4 && self.gen_num > 3{
            return true;
        } else if self.grid == self.grid5 && self.gen_num > 4{
            return true;
        } else {
            return false;
        }
    }
    //normal next generation
    pub fn next_gen(&mut self) {
        let mut next_gen = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut neighbors = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let neighbor_x = x as i32 + i;
                        let neighbor_y = y as i32 + j;
                        if neighbor_x < 0 || neighbor_y < 0 || neighbor_x >= self.width as i32 || neighbor_y >= self.height as i32 {
                            continue;
                        }
                        if self.grid[neighbor_y as usize][neighbor_x as usize] == 'X' {
                            neighbors += 1;
                        }
                    }
                }
                if self.grid[y][x] == 'X' {
                    if neighbors < 2 || neighbors > 3 {
                        next_gen[y][x] = '-';
                    }
                } else {
                    if neighbors == 3 {
                        next_gen[y][x] = 'X';
                    }
                }
            }
        }
        self.grid5 = self.grid4.clone();
        self.grid4 = self.grid3.clone();
        self.grid3 = self.grid2.clone();
        self.grid2 = self.grid.clone();
        self.gen_num += 1;
        self.grid = next_gen;
    }
    //wrap around next generation
    pub fn next_gen_wrap(&mut self) {
        let mut next_gen = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut neighbors = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let neighbor_x = (x as i32 + i + self.width as i32) % self.width as i32;
                        let neighbor_y = (y as i32 + j + self.height as i32) % self.height as i32;
                        if self.grid[neighbor_y as usize][neighbor_x as usize] == 'X' {
                            neighbors += 1;
                        }
                    }
                }
                if self.grid[y][x] == 'X' {
                    if neighbors < 2 || neighbors > 3 {
                        next_gen[y][x] = '-';
                    }
                } else {
                    if neighbors == 3 {
                        next_gen[y][x] = 'X';
                    }
                }
            }
        }
        self.grid5 = self.grid4.clone();
        self.grid4 = self.grid3.clone();
        self.grid3 = self.grid2.clone();
        self.grid2 = self.grid.clone();
        self.gen_num += 1;
        self.grid = next_gen;
    }
    //mirror next generation
    pub fn next_gen_mirror(&mut self) {
        let mut next_gen = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut neighbors = 0;
                for i in -1..2 {
                    for j in -1..2 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let neighbor_x = if x as i32 + i < 0 {
                            0
                        } else if x as i32 + i >= self.width as i32 {
                            self.width as i32 - 1
                        } else {
                            x as i32 + i
                        };
                        let neighbor_y = if y as i32 + j < 0 {
                            0
                        } else if y as i32 + j >= self.height as i32 {
                            self.height as i32 - 1
                        } else {
                            y as i32 + j
                        };
                        if self.grid[neighbor_y as usize][neighbor_x as usize] == 'X' {
                            neighbors += 1;
                        }
                    }
                }
                if self.grid[y][x] == 'X' {
                    if neighbors < 2 || neighbors > 3 {
                        next_gen[y][x] = '-';
                    }
                } else {
                    if neighbors == 3 {
                        next_gen[y][x] = 'X';
                    }
                }
            }
        }
        self.grid5 = self.grid4.clone();
        self.grid4 = self.grid3.clone();
        self.grid3 = self.grid2.clone();
        self.grid2 = self.grid.clone();
        self.gen_num += 1;
        self.grid = next_gen;
    }
}

// game of life start game
pub fn start_game() {
    print!("Welcome to game of life! \n");
    print!("What gamemode would you like to play? \n");
    print!("1. Normal \n");
    print!("2. Wrap Around \n");
    print!("3. Mirror \n");
    let mut gamemode = String::new();
    io::stdin().read_line(&mut gamemode).expect("Failed to read line");
    let gamemode: u32 = gamemode.trim().parse().expect("Please type a number!");
    print!("What size would you like the grid to be? \n");
    print!("Width: \n");
    let mut width = String::new();
    io::stdin().read_line(&mut width).expect("Failed to read line");
    let width: u32 = width.trim().parse().expect("Please type a number!");
    print!("Height: \n");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: u32 = height.trim().parse().expect("Please type a number!");
    let mut grid = GoLGrid::new(width as usize, height as usize);
    //populate the grid
    grid.randomize();
    print!("Starting Simulation... \n");
    //run generations
    //open file
    let mut file = File::create("output.txt").expect("Unable to create file");
    while grid.get_gen_num() < 1000 && !grid.check_match(){
        match gamemode {
            1 => grid.next_gen(),
            2 => grid.next_gen_wrap(),
            3 => grid.next_gen_mirror(),
            _ => grid.next_gen(),
        }
        print!("{}", grid.return_grid());
        file.write_all(grid.return_grid().as_bytes()).expect("Unable to write data");
    }
    print!("Simulation Complete! \n");
}
