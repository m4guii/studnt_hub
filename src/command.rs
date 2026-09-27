use crate::program::*;
use crate::grades::*;
use crate::todo::*;

pub fn launch(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics[0].to_string() != "Hub" && specifics[0].to_string() != "Record" {
        println!("Invalid menu. Try again."); 
    } else {
        program.choose_menu(specifics[0].to_string()); 
        program.show_menu();
    }
}

pub fn quit(program: &mut Program) -> () {
    println!("Program is closing..."); 
    program.on = 0; 
}

pub fn create_father(program: &mut Program, specifics: Vec<&str>) -> () {
    if program.menu == "Hub" {
        let new_to_do_list = create_todo_list(specifics[0].to_string()); 
        program.todo_lists.push(new_to_do_list); 
        println!("You have created the To Do List '{}'.", specifics[0].to_string()); 
    } else if program.menu == "Record" {
        let new_year = create_year(specifics[0].to_string()); 
        program.years.push(new_year); 
        println!("You have created the Year '{}'.", specifics[0].to_string()); 
    } else {
        println!("Choose a menu first."); 
    }
}

pub fn create_son(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 2 {
        println!("Not enough arguments. Try again."); 
        return; 
    }
    if program.menu == "Hub" {
        if let Some(todolist) = program.find_todo_list(specifics[0].to_string()) {
            todolist.add_todo(specifics[1].to_string()); 
        } else {
            println!("Consider creating the To Do List '{}' first.", specifics[0]); 
        }
        println!("You have created the To Do Item '{}' in the To Do List '{}'.", specifics[1], specifics[0]); 
    } else if program.menu == "Record" {
        if let Some(year) = program.find_year(specifics[0].to_string()) {
            year.create_subject(specifics[1].to_string(), 0, false, 0); 
        } else {
            println!("Consider creating the Year '{}' first.", specifics[0]); 
        }
        println!("You have created the Subject Item '{}' in the Year '{}'.", specifics[1], specifics[0]); 
    } else {
        println!("Choose a menu first."); 
    }
}

pub fn delete_father(program: &mut Program, specifics: Vec<&str>) -> () {

}

pub fn delet_son(program: &mut Program, specifics: Vec<&str>) -> () {

}

pub fn display(program: &mut Program, specifics: Vec<&str>) -> () {

}

pub fn click(program: &mut Program, specifics: Vec<&str>) -> () {

}

pub fn grade(program: &mut Program, specifics: Vec<&str>) -> () {

}

pub fn ects(program: &mut Program, specifics: Vec<&str>) -> () {

}

pub fn error() -> () {
    println!("Invalid command. To open the manual, type 'man'.");
}