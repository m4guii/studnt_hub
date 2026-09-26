use crate::program::*;

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

}

pub fn create_son(program: &mut Program, specifics: Vec<&str>) -> () {

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