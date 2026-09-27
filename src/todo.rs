pub struct ToDoList {
    pub title: String,
    done_counter: u8, 
    total_counter: u8,
    todos: Vec<ToDo>,
}

pub struct ToDo {
    title: String, 
    done: bool, 
}

pub fn create_todo_list(list_name: String) -> ToDoList {
    let todo_list: Vec<ToDo> = Vec::new(); 
    ToDoList { title: list_name, done_counter: 0, total_counter: 0, todos: todo_list }
}

impl ToDoList {
    pub fn add_todo(&mut self, todo: String) -> () {
        self.todos.push(ToDo {title: todo, done: false,}); 
        self.total_counter += 1; 
    }

    pub fn delete_todo(&mut self, todo: &str) -> () {
        self.todos.retain(|item| item.title != todo);
        self.total_counter -= 1; 
    }

    pub fn check_todo(&mut self, todo: &str) -> () {
        for item in &mut self.todos {
            if item.title == todo {
                item.done = true;
            }
        }
        self.done_counter += 1; 
    }

    pub fn uncheck_todo(&mut self, todo: &str) -> () {
        for item in &mut self.todos {
            if item.title == todo {
                item.done = false;
            }
        }
        self.done_counter -= 1; 
    }

    pub fn delete_todo_list(self) -> () {
        drop(self); 
    }

    pub fn get_progress_list(self) -> f32 {
        if self.total_counter == 0 {
            return 0.0
        }
        (self.done_counter as f32 / self.total_counter as f32) * 100.0
    }

    pub fn print_todo_list(self) -> () {
        println!("=== {} ===", self.title); 
        for item in &self.todos {
            println!("{} : {}", item.title, if item.done == true {"v"} else {"x"});
        }
        println!("{}", self.get_progress_list());
        println!("==================="); 
    }

}

