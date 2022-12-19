#![warn(rust_2018_idioms)]

pub mod grid;
pub mod node;
use crate::grid::Grid;

use colored::control;

use core::panic;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn run() {
    control::set_override(true);
    // Read in file
    // read command line arguments for the filename
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() >= 3 {
        let mut grid: Grid;

        let x_size;
        let y_size;

        match args[1].parse::<usize>() {
            Ok(i) => x_size = i,
            Err(err) => panic!("{}", err),
        }
        match args[2].parse::<usize>() {
            Ok(i) => y_size = i,
            Err(err) => panic!("{}", err),
        }

        if args.len() == 4 {
            // &args[1] is the filename
            let map_vec = lines_from_file(&args[3]);
            grid = Grid::from(map_vec, x_size, y_size);
        }
        else {
            grid = Grid::generate_random_grid(x_size, y_size);
        }
        
        println!("Unsolved Map: ");
        grid.print_grid();
        println!("\nSolved Map: ");
        grid.run_a_star(true);
    }
    else {
        println!("No arguments were given, please try again with arguments in this format.");
        println!("pathfinder [x_size] [y_size] [filename <optional>]");
    }
    
    colored::control::unset_override();
}

fn main() {
    #[cfg(windows)]
    match control::set_virtual_terminal(true){
        Ok(_i) => {},
        Err(_err) => panic!("Couldn't set virtual terminal"),
    }

    run();
}
