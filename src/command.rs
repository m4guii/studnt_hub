use crate::program::*;

pub fn launch(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 1 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }
    if specifics[0].to_string() != "Hub" && specifics[0].to_string() != "Record" {
        println!("Oops! Invalid menu. Try again."); 
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
    if specifics.len() != 1 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }
    
    if program.menu == "Hub" {
        program.create_todo_list(specifics[0].to_string());
    } else if program.menu == "Record" {
        program.create_year(specifics[0].to_string());
    } else {
        println!("Oops! Suggestion: Choose a menu first."); 
    }
}

pub fn create_son(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 2 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }
    if program.menu == "Hub" {
        if let Some(todolist) = program.find_todo_list(specifics[0].to_string()) {
            todolist.add_todo(specifics[1].to_string()); 
        }
    } else if program.menu == "Record" {
        if let Some(year) = program.find_year(specifics[0].to_string()) {
            year.create_subject(specifics[1].to_string(), 0, false, 0); 
        }
    } else {
        println!("Oops! Suggestion: Choose a menu first."); 
    }
}

pub fn delete_father(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 1 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }
    if program.menu == "Hub" {
        program.delete_todo_list(specifics[0].to_string());
    } else if program.menu == "Record" {
        program.delete_year(specifics[0].to_string());
    } else {
        println!("Oops! Suggestion: Choose a menu first."); 
    }
}

pub fn delete_son(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 2 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }
    if program.menu == "Hub" {
        program.delete_todo(specifics[0].to_string(), specifics[1].to_string());
    } else if program.menu == "Record" {
        program.delete_subject(specifics[0].to_string(), specifics[1].to_string());
    } else {
        println!("Oops! Suggestion: Choose a menu first."); 
    }
}

pub fn display(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 1 &&  specifics.len() != 0 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }

    if specifics.len() == 1 && program.menu == "Hub" {
        if let Some(todolist) = program.find_todo_list(specifics[0].to_string()) {
            todolist.print_todo_list();
        }
    } else if specifics.len() == 1 && program.menu == "Record" {
        if let Some(year) = program.find_year(specifics[0].to_string()) {
            year.print_year();
        }
    } else if specifics.len() == 0 {
        program.show_menu();
    } else {
        println!("Oops! Suggestion: Choose a menu first."); 
    }
}

pub fn click(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 2 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }

    if program.menu == "Hub" {
        if let Some(todolist) = program.find_todo_list(specifics[0].to_string()) {
            todolist.check_todo(specifics[1]);
            todolist.print_todo_list();
        }
    } else if program.menu == "Record" {
        if let Some(year) = program.find_year(specifics[0].to_string()) {
            year.check_sim(specifics[1].to_string());
            year.print_year();
        }
    } else {
        println!("Oops! Suggestion: Choose a menu first."); 
    }
}

pub fn grade(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 3 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }

    if program.menu != "Record" {
        println!("Oops! Suggestion: Check in which menu you are. This command only works in 'Record'.")
    } else {
        if let Some(year) = program.find_year(specifics[0].to_string()) {
            match specifics[2].parse::<u8>() {
                Ok(num) => year.edit_grade(specifics[1].to_string(), num),
                Err(e) => println!("Oops! '{}' is not a valid grade.", e),
            }
            year.print_year();
        }
    }
}

pub fn ects(program: &mut Program, specifics: Vec<&str>) -> () {
    if specifics.len() != 3 {
        println!("Oops! Suggestion: Check the number of arguments of this command in 'man'."); 
        return; 
    }

    if program.menu != "Record" {
        println!("Oops! Suggestion: Check in which menu you are. This command only works in 'Record'.")
    } else {
        if let Some(year) = program.find_year(specifics[0].to_string()) {
            match specifics[2].parse::<u8>() {
                Ok(num) => year.edit_ects(specifics[1].to_string(), num),
                Err(e) => println!("Oops! '{}' is not a valid grade.", e),
            }
            year.print_year();
        }
    }
}

pub fn error() -> () {
    println!("Invalid command. Suggestion: Type 'man' to check possible commands.");
}