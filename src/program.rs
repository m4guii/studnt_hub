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

    pub fn find_todo_list(&mut self, list_name: String) -> Option<&mut ToDoList> {
        let list = self.todo_lists.iter_mut().find(|item| item.title == list_name);
        if list.is_none() {
            println!("Consider creating the To Do List '{}' first.", list_name);
        }
        list
    }

    pub fn find_year(&mut self, year_name: String) -> Option<&mut Year> {
        let year = self.years.iter_mut().find(|item| item.name == year_name);
        if year.is_none() {
            println!("Consider creating the To Do List '{}' first.", year_name);
        }
        year
    }
}