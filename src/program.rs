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
            println!("Oops! Suggestion: Consider creating the To Do List '{}' first.", list_name);
        }
        list
    }

    pub fn find_year(&mut self, year_name: String) -> Option<&mut Year> {
        let year = self.years.iter_mut().find(|item| item.name == year_name);
        if year.is_none() {
            println!("Oops! Suggestion: Consider creating the Year '{}' first.", year_name);
        }
        year
    }

    pub fn create_todo_list(&mut self, todolist: String) -> () {
        let new_to_do_list = create_todo_list(todolist.to_string()); 
        self.todo_lists.push(new_to_do_list); 
        println!("You have created the To Do List '{}'.", todolist); 
    }

    pub fn create_year(&mut self, year: String) -> () {
        let new_year = create_year(year.to_string()); 
        self.years.push(new_year); 
        println!("You have created the Year '{}'.", year); 
    }

    pub fn delete_year(&mut self, year: String)-> () {
        self.years.retain(|item| item.name != year);
        println!("You have deleted all instances of the Year '{}'.", year); 
    }

    pub fn delete_todo_list(&mut self, todolist: String) -> () {
        self.todo_lists.retain(|item| item.title != todolist);
        println!("You have deleted all instances of the To Do List '{}'.", todolist); 
    }

    pub fn delete_todo(&mut self, todolist: String, todo: String) -> () {
        if let Some(todo_list) = self.find_todo_list(todolist.to_string()) {
            todo_list.byetodo(&todo);
            println!("You have deleted all instances of the To Do Item '{}'.", todo);
        } else {
            println!("Oops! To Do List '{}' not found.", todolist);
        }
    }

    pub fn delete_subject(&mut self, year: String, subject: String) -> () {
        if let Some(found_year) = self.find_year(year.to_string()) {
            found_year.byesubject(subject.to_string());
            println!("You have deleted all instances of the To Do Item '{}'.", subject);
        } else {
            println!("Oops! To Do List '{}' not found.", year);
        }
    }

}