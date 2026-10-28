# STUDENT HUB

Student Hub is a terminal-based productivity application built with Rust. It provides a centralized space to manage study timetables, stay focused with a Pomodoro timer, and track academic progress—including real-time grade average calculations and grade simulation features.

---

## Overview & Features

Student Hub is split into three main operational menus:

* **The Hub**: Manage your timetables, task lists, and daily to-dos.
* **The Timer**: Integrated Pomodoro timer to help manage focused work sessions. (TBD)
* **The Record**: Store academic grades and ECTS credits, calculate overall averages, and run grade simulations for future terms.

---

## Built With

* **Rust** - Systems programming language providing speed and safety.
* **Cargo** - Rust's standard build tool and package manager.

---

## Getting Started

### Prerequisites
Make sure you have Rust and Cargo installed on your system. If not, install them via rustup.rs:
```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
```

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/m4guii/studnt_hub.git
    ```

2. Navigate to the project directory:
   ```bash
   cd studnt_hub
   ```

### Running the Application

To build and launch Student Hub directly:
```bash
cargo run
```

To compile without running:
```bash
cargo build
```

---

## Project Structure

```text
studnt_hub/
├── src/
│   ├── main.rs       # Application entry point
│   ├── command.rs    # Command handling and dispatching
│   ├── program.rs    # Application state and navigation
│   ├── todo.rs       # Task and to-do list logic
│   ├── grades.rs     # Academic records and averages
│   └── tools.rs      # Input parsing and utility functions
├── Cargo.toml        # Project configuration and dependencies
├── Cargo.lock        # Locked dependency versions
└── README.md
```

---

## Structs & Data Architecture

| Struct Name | Type | Description |
| :--- | :--- | :--- |
| **To-Do List** | Father | Container with list title, total item counter, completed counter, and a vector of To-Do Items. |
| **To-Do Item** | Son | Task element containing the item name and completion status (done / not done). |
| **Subject Item** | Son | Subject record containing the name, ECTS credits, S/T ratio, and grade. |
| **Year Item** | Father | Academic year container with year name, vector of Subject Items, and overall yearly average. |
| **Program** | Core State | Central program state containing navigation data and the active menu. |

---

## API / Module Reference

### command.rs
* launch(program: &mut Program, specifics: Vec<&str>) — Launches a new program view.
* quit(program: &mut Program) — Quits the current program context.
* create_father(program: &mut Program, specifics: Vec<&str>) — Creates a To-Do List (on Hub) or a Year (on Record).
* create_son(program: &mut Program, specifics: Vec<&str>) — Creates a To-Do Item (on Hub) or a Subject Item (on Record).
* delete_father(program: &mut Program, specifics: Vec<&str>) — Removes a To-Do List or Year.
* delete_son(program: &mut Program, specifics: Vec<&str>) — Removes a To-Do Item or Subject Item.
* display(program: &mut Program, specifics: Vec<&str>) — Displays list/year details or main menu navigation.
* click(program: &mut Program, specifics: Vec<&str>) — Toggles task completion (on Hub) or subject simulation status (on Record).
* grade(program: &mut Program, specifics: Vec<&str>) — Updates the grade of a Subject Item.
* ects(program: &mut Program, specifics: Vec<&str>) — Updates ECTS credits for a Subject Item.
* error() — Dispatches standard error handling.

### todo.rs
* ToDo Methods:
  * add_todo(&mut self, todo: String) — Appends a new item to a list.
  * byetodo(&mut self, todo: &str) — Removes an item from a list.
  * check_todo(&mut self, todo: &str) — Toggles task completion status.
  * get_progress_list(&self) -> f32 — Returns percentage completion rate.
  * print_todo_list(&self) — Renders the task list to stdout.
* create_todo_list(list_name: String) -> ToDoList — Factory function to instantiate a list.

### grades.rs
* Year Methods:
  * create_subject(&mut self, subject: String, credits: u8, sim: bool, grade: u8) — Adds a subject record.
  * edit_grade(&mut self, subject: String, grade: u8) — Edits course grade.
  * edit_ects(&mut self, subject: String, ects: u8) — Edits course ECTS credits.
  * check_sim(&mut self, subject: String) — Toggles simulation status for grade calculation.
  * byesubject(&mut self, subject: String) — Deletes a subject record.
  * yearly_average(&mut self) -> f32 — Computes weighted yearly average.
  * print_year(&mut self) — Renders year data to stdout.
* create_year(year: String) -> Year — Instantiates a new year record.
* total_avg(years: &[Year]) -> f32 — Computes global GPA across all active years.

### program.rs
* choose_menu(&mut self, choice: String) — Switches active menu context.
* show_menu(&mut self) — Displays active menu interface.
* end(self) — Safely terminates program state.
* find_todo_list(&mut self, list_name: String) -> Option<&mut ToDoList> — Searches for list by name.
* find_year(&mut self, year_name: String) -> Option<&mut Year> — Searches for academic year by name.
* Entity management helper functions: create_todo_list, create_year, delete_year, delete_todo_list, delete_todo, delete_subject.

### tools.rs
* man() — Prints command manual to stdout.
* select(input: &String, program: &mut Program) — Parses input line and invokes corresponding commands.
* select_first(input: &str) -> String — Extracts primary command token.
* parse(input: &str) -> Vec<&str> — Splits command argument string.

### main.rs
* main() — Program initialization and application loop execution.

---

## Roadmap

1. Phase 1 (Current): Terminal I/O interface. (DONE)
2. Phase 2: File persistence & database output integration. (TBD)
3. Phase 3: Full GUI interface using WebSockets, HTML, and CSS. (TBD)

---

## Contributing

Suggestions, bug reports, and contributions are welcome!

1. Fork the Project
2. Create your Feature Branch (\`git checkout -b feature/NewFeature\`)
3. Commit your Changes (\`git commit -m 'Add NewFeature'\`)
4. Push to the Branch (\`git push origin feature/NewFeature\`)
5. Open a Pull Request

---

## Notes

* Built as a personal project to practice Rust concepts.
