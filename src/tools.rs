use crate::command::*; 
use crate::program::*;

pub fn man() -> () {
    println!(" ");
    println!("This is the manual. Here are described all commands accepted by this program. <> are the parameters you should add to the command mandatorily.");
    println!("      s <menu> := starts program in 'menu', or changes current menu to 'menu'.");
    println!("      Menu Options: Hub (1), Record (2).");
    println!(" ");
    println!("      q := ends program.");
    println!(" ");
    println!("--------------------------------------------------------"); 
    println!(" ");
    println!("Once you have chosen a menu, your functionalities are restricted to that menu.");
    println!(" ");
    println!("      c <To-Do List / Year> := if on 'Hub' menu, creates a To-Do List with the input as title; if on 'Record' menu, creates a Year with the input as title.");
    println!(" ");
    println!("      rm <To-Do / Subject> := if on 'Hub' menu, deletes 'To-Do' from its To-Do List; if on 'Record' menu, deletes 'Subject' from its Year.");
    println!(" ");
    println!("      del <To-Do List / Year> := if on 'Hub' menu, deletes 'To-Do-List'; if on 'Record' menu, deletes 'Year'.");
    println!(" ");
    println!("      cs <To-Do List / Year> <To-Do / Subject> := creates 'To-Do' item in 'To-Do List' if in Hub, if in Record creates 'Subject' in 'Year'.");
    println!(" ");
    println!("      display <To-Do List / Year> := if on 'Hub' menu, displays 'To-Do-List' content; if on 'Record' menu, displays 'Year' content.");
    println!(" ");
    println!("      click <To-Do / Subject> := checks / unchecks 'To-Do' if in Hub; if in Record, checks / unchecks Simulation box.");
    println!(" ");
    println!("--------------------------------------------------------"); 
    println!(" ");
    println!("Hub specific commands:"); 
    println!(" ");
    println!("      None.");
    println!(" ");
    println!("--------------------------------------------------------"); 
    println!(" ");
    println!("Record specific commands:"); 
    println!(" ");
    println!("      g <Subject> <Grade> := changes grade of 'Subject' to 'Grade'.");
    println!(" ");
    println!("      e <Subject> <Ects> := changes ects of 'Subject' to 'Ects'.");
    println!(" ");
    println!("--------------------------------------------------------"); 
    println!(" ");
    println!("SPECIAL NOTES:");
    println!(" ");
    println!("  -> When creating a To-Do item, it is unchecked by default."); 
    println!(" ");
    println!("  -> When creating a Subject item, it is, by default, unchecked in Simulation box, the grade is 0, and the ects are 0."); 
    println!(" ");
    println!("  -> When creating a To-Do List or a Year, they are empty by default."); 
    println!(" ");
    println!("  -> All specifics from commands inside <> MUST be between quote marks."); 
    println!(" ");
}

pub fn select(input: &String, program: &mut Program) -> () {
    let cmd = select_first(input);
    let specifics = parse(input);

    match cmd.as_str() {
        "man" => man(), 
        "s" => launch(program, specifics), 
        "q" => quit(program), 
        "c" => create_father(program, specifics), 
        "cs" => create_son(program, specifics), 
        "del" => delete_father(program, specifics), 
        "rm" => delet_son(program, specifics), 
        "display" => display(program, specifics), 
        "click" => click(program, specifics), 
        "g" => grade(program, specifics), 
        "e" => ects(program, specifics), 
        _ => error(),
    }
}

pub fn select_first(input: &str) -> String {
    input.split_whitespace().next().unwrap_or("").to_string()
}

pub fn parse(input: &str) -> Vec<&str> {
    let trimmed = input.trim();

    if trimmed.contains('"') {
        trimmed
            .split('"')
            .enumerate()
            .filter(|(index, _)| index % 2 == 1)
            .map(|(_, text)| text)
            .collect()
    } else {
        trimmed
            .split_whitespace()
            .skip(1) 
            .collect()
    }
}
