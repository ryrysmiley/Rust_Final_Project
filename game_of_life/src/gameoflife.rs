/*Game of Life Grid*/
extern crate rand;
use std::io::{self, Write};
use std::collections::HashSet;
use::colored::*;
// use std::fs::File;
// use std::io::Write;
pub struct GoLGrid {
    grid: Vec<Vec<char>>,
    library: HashSet<Vec<Vec<char>>>,
    gen_num: usize,
    width: usize,
    height: usize,
    finished_grids: Vec<Vec<Vec<char>>>,
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
            library: HashSet::new(),
            gen_num: 1,
            width: width,
            height: height,
            finished_grids: Vec::new(),
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
    //print return grid given the index
    pub fn print_grid_final(&self, index: usize) {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.finished_grids[index][y][x]);
            }
            output.push('\n');
        }
        print!("{}{}", output, format!("Generation: {} \n", index + 1).blue().bold());
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
        //check hashset
        if self.library.contains(&self.grid) {
            return true;
        }
        return false;
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
        self.finished_grids.push(self.grid.clone());
        self.library.insert(self.grid.clone());
        self.grid = next_gen;
        self.gen_num += 1;
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
        self.finished_grids.push(self.grid.clone());
        self.library.insert(self.grid.clone());
        self.grid = next_gen;
        self.gen_num += 1;
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
        self.finished_grids.push(self.grid.clone());
        self.library.insert(self.grid.clone());
        self.grid = next_gen;
        self.gen_num += 1;
    }
}

// game of life start game
// pub fn start_game() {
//     print!("Welcome to game of life! \n");
//     print!("What gamemode would you like to play? \n");
//     print!("1. Normal \n");
//     print!("2. Wrap Around \n");
//     print!("3. Mirror \n");
//     let mut gamemode = String::new();
//     io::stdin().read_line(&mut gamemode).expect("Failed to read line");
//     let gamemode: u32 = gamemode.trim().parse().expect("Please type a number!");
//     print!("What size would you like the grid to be? \n");
//     print!("Width: \n");
//     let mut width = String::new();
//     io::stdin().read_line(&mut width).expect("Failed to read line");
//     let width: u32 = width.trim().parse().expect("Please type a number!");
//     print!("Height: \n");
//     let mut height = String::new();
//     io::stdin().read_line(&mut height).expect("Failed to read line");
//     let height: u32 = height.trim().parse().expect("Please type a number!");
//     let mut grid = GoLGrid::new(width as usize, height as usize);
//     //populate the grid
//     grid.randomize();
//     print!("Starting Simulation... \n");
//     //run generations
//     //open file
//     let mut file = File::create("output.txt").expect("Unable to create file");
//     while grid.get_gen_num() < 1000 && !grid.check_match(){
//         match gamemode {
//             1 => grid.next_gen(),
//             2 => grid.next_gen_wrap(),
//             3 => grid.next_gen_mirror(),
//             _ => grid.next_gen(),
//         }
//         print!("{}", grid.return_grid());
//         file.write_all(grid.return_grid().as_bytes()).expect("Unable to write data");
//     }
//     print!("Simulation Complete! \n");
// }

// parameters are gamemode, width, height
pub fn full_game() {
    //get user input and keep asking until valid
    print!("{esc}c", esc = 27 as char);
    print!("{}", "[Game of Life] \n".green().bold());
    //enter gamemode as integer
    //Ask until input is valid
    let mut gamemode = String::new();
    print!("{}","[What gamemode would you like to play?] \n".red().bold());
    'gm: loop {
        print!("1. Normal \n");
        print!("2. Wrap Around \n");
        print!("3. Mirror \n");
        gamemode.clear();
        io::stdout().flush();
        io::stdin().read_line(&mut gamemode).unwrap();
        let mut gamemode = gamemode.trim().to_string();
        print!("{esc}c", esc = 27 as char);
        match gamemode.parse::<u32>() {
            Ok(i) => if i > 0 && i < 4 {break 'gm} else {println!("{}{}","[Game of Life] \n".green().bold(), "[Please enter a valid gamemode! (1,2,3)]".red().bold())},
            Err(..) => print!("{}{}","[Game of Life] \n".green().bold(), "[Please enter a valid gamemode! (1,2,3)] \n".red().bold()),
        };
    }

    //enter width and height as integers
    //Ask until input is valid
    let mut width = String::new();
    print!("{}", "[Game of Life] \n".green().bold());
    print!("{}","[What width would you like the grid to be?] \n".red().bold());
    'w: loop {
        width.clear();
        io::stdout().flush();
        io::stdin().read_line(&mut width).unwrap();
        let mut width = width.trim().to_string();
        print!("{esc}c", esc = 27 as char);
        match width.parse::<u32>() {
            Ok(i) => if i > 0 && i < 101 {break 'w} else {println!("{}{}","[Game of Life] \n".green().bold(), "[Please enter a valid width! (1-100)]".red().bold())},
            Err(..) => print!("{}{}","[Game of Life] \n".green().bold(), "[Please enter a valid width! (1-200)] \n".red().bold()),
        };
    }
    let mut height = String::new();
    print!("{}", "[Game of Life] \n".green().bold());
    print!("{}","[What height would you like the grid to be?] \n".red().bold());
    'h: loop {
        height.clear();
        io::stdout().flush();
        io::stdin().read_line(&mut height).unwrap();
        let mut height = height.trim().to_string();
        print!("{esc}c", esc = 27 as char);
        match height.parse::<u32>() {
            Ok(i) => if i > 0 && i < 51 {break 'h} else {println!("{}{}","[Game of Life] \n".green().bold(), "[Please enter a valid height! (1-50)]".red().bold())},
            Err(..) => print!("{}{}","[Game of Life] \n".green().bold(), "[Please enter a valid height! (1-200)] \n".red().bold()),
        };
    }

    //convert gamemode width height to u32
    let gamemode: u32 = gamemode.trim().parse().expect("Error with conversion");
    let width: u32 = width.trim().parse().expect("Error with conversion");
    let height: u32 = height.trim().parse().expect("Error with conversion");

    //create grid
    let mut grid = GoLGrid::new(width as usize, height as usize);
    //populate the grid
    grid.randomize();
    //run generations
    while grid.get_gen_num() < 1000 {
        match gamemode {
            1 => grid.next_gen(),
            2 => grid.next_gen_wrap(),
            3 => grid.next_gen_mirror(),
            _ => grid.next_gen(),
        }
        if grid.check_match() {
            break;
        }
    }
    //using the 3D vector allow a user to look at different generations
    let genIndex = 0;
    'v: loop {
        print!("{esc}c", esc = 27 as char);
        print!("{}", "[Game of Life] \n".green().bold());
        print!("{}{}{}{}{}{}{}{}{}",format!("[Gamemode: ").red().bold(),format!("{}",gamemode).red().bold().bold(),format!("]").red(),format!("[Width: ").red().bold(),format!("{}",width).red().bold().bold(),format!("]").red(),format!("[Height: ").red().bold(),format!("{}",height).red().bold().bold(),format!("]\n").red());
        //print the grid
        grid.print_grid_final(genIndex);
        //user options
        break;
    }
}
