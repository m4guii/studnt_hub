# ==== STUDENT HUB ====
This code is for an application with three menus: 
* the hub ( for timetables + to-dos );
* the timer ( with pomodoro timer );
* the record ( for keeping record of your grades as well as simulate future averages).
<br>
This code will be produced in RUST; <br>
This code will first be created for terminal I/O -> then converted to a file output ( database only ) -> then converted to an UI using websockets & HTML / CSS. <br>
This page will hold info on the files of the project, as well as functions, with their expected inputs and outputs, errors and goals. <br>

## Structs
    to-do list item - struct with vec of structs:to-do, title, to-do counter and dones counter;
    to-do items - struct with name, done/not done;

    subject item - struct with name, ects, s/t and grade;
    year item - struct with name, vec of structs:subjects, average;

    program - struct with name of menu it is currently in; 


## Files

### command
    launch(program: &mut Program, specifics: Vec<&str>) -> ()
        - launches new program; 

    quit(program: &mut Program) -> ()
        - quits current program; 

    create_father(program: &mut Program, specifics: Vec<&str>) -> ()
        - creates a To Do List, if on Hub. If on Record, creates a Year; 
        
    create_son(program: &mut Program, specifics: Vec<&str>) -> ()
        - creates a To Do Item, if on Hub. If on Record, creates a Subject Item; 
        
    delete_father(program: &mut Program, specifics: Vec<&str>) -> ()
        - deletes a To Do List, if on Hub. If on Record, deletes a Year; 
        
    delete_son(program: &mut Program, specifics: Vec<&str>) -> ()
        - deletes a To Do Item, if on Hub. If on Record, deletes a Subject Item; 
        
    display(program: &mut Program, specifics: Vec<&str>) -> ()
        - displays a To Do List, if on Hub. If on Record, displays a Year. It also can display the Menu;
        
    click(program: &mut Program, specifics: Vec<&str>) -> ()
        - checks the Done checkbox of a To Do Item, if on Hub. If on Record, checks the Simulation checkbox of a Subject Item; 
        
    grade(program: &mut Program, specifics: Vec<&str>) -> ()
        - changes the Grade of a Subject Item; 
        
    ects(program: &mut Program, specifics: Vec<&str>) -> ()
        - changes the ECTS of a Subject Item; 
        
    error() -> ()
        - finds an error.
        
### todo 

### main

### grades

### program

### toold



