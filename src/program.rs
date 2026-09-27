use crate::grades::*;
use crate::todo::*;
pub struct Program {
    pub menu: String,
    pub on: u8,
    pub todo_lists: Vec<ToDoList>,
    pub years: Vec<Year>,
}

impl Program {
    pub fn choose_menu(&mut self, choice: String) -> () {
        self.menu = choice; 
    }

    pub fn show_menu(&mut self) -> () {
        println!("Current menu is {}.", self.menu); 
    }

    pub fn end(self) -> () {
        drop(self); 
        println!("End.");
        std::process::exit(0);
    }
}