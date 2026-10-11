# ==== STUDENT HUB ====
This code is for an application with three menus: 
* the hub ( for timetables + to-dos );
* the timer ( with pomodoro timer );
* the record ( for keeping record of your grades as well as simulate future averages).
<br>
This code will be produced in RUST; <br>
This code was produced for the purpose of practicing the Rust Language; <br>
This code will first be created for terminal I/O -> then converted to a file output ( database only ) -> then converted to an UI using websockets & HTML / CSS. <br>
This page will hold info on the files of the project, as well as functions, with their expected inputs and outputs, errors and goals. <br>
This code was produced for the purpose of practicing the Rust Language. 

## Structs
    to-do list item - struct with vec of structs:to-do, title, to-do counter and dones counter;
    to-do items - struct with name, done/not done;

    subject item - struct with name, ects, s/t and grade;
    year item - struct with name, vec of structs:subjects, average;

    program - struct with name of menu it is currently in; 


## Files

### command.rs
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
        
### todo.rs
    ToDo: 
        add_todo(&mut self, todo: String) -> ()
            - adds a To Do Item to a To Do List; 
 
        byetodo(&mut self, todo: &str) -> ()
            - removes a To Do Item from a To Do List; 
 
        check_todo(&mut self, todo: &str) -> ()
            - checks a To Do Item of a To Do List; 
 
        get_progress_list(&self) -> f32
            - returns the progress of a To Do List; 
 
        print_todo_list(&self) -> ()
            - prints To Do List;
 
    create_todo_list(list_name: String) -> ToDoList 
            - creates a To Do List; 
 
### grades.rs
    Year: 
        create_subject(&mut self, subject: String, credits: u8, sim: bool, grade: u8) -> ()
            - adds a Subject Item to a Year; 
     
        edit_grade(&mut self, subject: String, grade: u8) -> ()
            - edits Grade of Subject Item of a Year; 
     
        edit_ects(&mut self, subject: String, ects: u8) -> ()
            - edits ECTS of Subject Item of a Year; 
     
        check_sim(&mut self, subject: String) -> ()
            - (un)checks the Simulation checkbox of Subject Item of a Year; 
     
        byesubject(&mut self, subject: String) -> ()
            - removes a Subject Item from a Year; 
     
        yearly_average(&mut self) -> f32
            - calculates yearly average; 
     
        print_year(&mut self) -> ()
            - prints Year
     

    create_year(year: String)-> Year
        - cerates a Year; 
     
    total_avg(years: &[Year]) -> f32
        - calculates total average of all Years in launched program; 

### program.rs
    Program: 
        choose_menu(&mut self, choice: String) -> ()
            - chooses menu (Hub / Record); 

        show_menu(&mut self) -> ()
            - displays menu; 
            
        end(self) -> ()
            - ends program; 
            
        find_todo_list(&mut self, list_name: String) -> Option<&mut ToDoList>
            - finds To Do List in Program; 
            
        find_year(&mut self, year_name: String) -> Option<&mut Year>
            - finds Year in Program; 
            
        create_todo_list(&mut self, todolist: String) -> ()
            - creates To Do List in Program; 
            
        create_year(&mut self, year: String) -> ()
            - creates Year in Program; 
            
        delete_year(&mut self, year: String)-> ()
            - deletes Year in Program; 
            
        delete_todo_list(&mut self, todolist: String) -> ()
            - deletes To Do List in Program; 
            
        delete_todo(&mut self, todolist: String, todo: String) -> ()
            - deletes To Do Item from To Do List in Program; 
            
        delete_subject(&mut self, year: String, subject: String) -> ()
            - deletes Subject Item from Year in Program; 

### tools.rs
    man() -> ()
        - displays manual of studnt_hub for users; 

    select(input: &String, program: &mut Program) -> ()
        - selects input : command + specifics; 
        
    select_first(input: &str) -> String
        - selects the first command in input string; 
        
    parse(input: &str) -> Vec<&str>
        - parses input; 
        

### main.rs

