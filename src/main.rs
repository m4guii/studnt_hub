use std::*; 
use crate::program::*; 
use crate::tools::*; 

mod todo; 
mod grades; 
mod tools;
mod program; 
mod command; 

fn main() {
    println!("Program has launched."); 
    let none = String::from("None."); 
    let mut program = Program { menu: none, on: 1, todo_lists: Vec::new(), years: Vec::new()}; 
    while program.on == 1 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
            
        select(&input, &mut program);
    }
}
