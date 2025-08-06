use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct Todo {
    id: usize,
    description: String,
    completed: bool,
    created_at: String,
}

#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }
}

#[derive(Debug)]
enum Command {
    Add(String),
    List,
    Done(usize),
    Delete(usize),
    Help,
    Exit,
    Unknown,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_file_path = get_data_file_path();
    let mut todo_list = load_from_file(&data_file_path)?;

    println!("Welcome to Todo CLI!");
    println!("Type 'help' for available commands.\n");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let command = parse_command(input);

        match command {
            Command::Add(description) => {
                add_todo(&mut todo_list, description);
                save_to_file(&todo_list, &data_file_path)?;
            }
            Command::List => {
                list_todos(&todo_list);
            }
            Command::Done(id) => {
                match complete_todo(&mut todo_list, id) {
                    Ok(_) => {
                        save_to_file(&todo_list, &data_file_path)?;
                        println!("Task {} marked as completed!", id);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Delete(id) => {
                match delete_todo(&mut todo_list, id) {
                    Ok(_) => {
                        save_to_file(&todo_list, &data_file_path)?;
                        println!("Task {} deleted!", id);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Help => {
                print_help();
            }
            Command::Exit => {
                println!("Goodbye!");
                break;
            }
            Command::Unknown => {
                println!("Unknown command. Type 'help' for usage.");
            }
        }
    }

    Ok(())
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Command::Unknown;
    }

    match parts[0] {
        "add" => {
            if parts.len() > 1 {
                let description = parts[1..].join(" ");
                if description.len() <= 500 {
                    Command::Add(description)
                } else {
                    Command::Unknown
                }
            } else {
                Command::Unknown
            }
        }
        "list" => Command::List,
        "done" => {
            if parts.len() == 2 {
                if let Ok(id) = parts[1].parse::<usize>() {
                    Command::Done(id)
                } else {
                    Command::Unknown
                }
            } else {
                Command::Unknown
            }
        }
        "delete" => {
            if parts.len() == 2 {
                if let Ok(id) = parts[1].parse::<usize>() {
                    Command::Delete(id)
                } else {
                    Command::Unknown
                }
            } else {
                Command::Unknown
            }
        }
        "help" => Command::Help,
        "exit" => Command::Exit,
        _ => Command::Unknown,
    }
}

fn add_todo(list: &mut TodoList, description: String) {
    let todo = Todo {
        id: list.next_id,
        description,
        completed: false,
        created_at: get_current_time(),
    };

    list.todos.push(todo);
    list.next_id += 1;

    println!("Added task with ID: {}", list.next_id - 1);
}

fn list_todos(list: &TodoList) {
    if list.todos.is_empty() {
        println!("No tasks found. Add one with 'add <description>'");
        return;
    }

    println!("\n=== TODO LIST ===");
    
    let mut completed_count = 0;
    let mut pending_count = 0;

    for todo in &list.todos {
        let status_symbol = if todo.completed { "[✓]" } else { "[ ]" };
        println!(
            "{} {} {} ({})",
            todo.id, status_symbol, todo.description, todo.created_at
        );

        if todo.completed {
            completed_count += 1;
        } else {
            pending_count += 1;
        }
    }

    println!("================");
    println!(
        "Total: {} | Completed: {} | Pending: {}",
        list.todos.len(),
        completed_count,
        pending_count
    );
    println!();
}

fn complete_todo(list: &mut TodoList, id: usize) -> Result<(), String> {
    if let Some(todo) = list.todos.iter_mut().find(|t| t.id == id) {
        if todo.completed {
            Err(format!("Task {} is already completed!", id))
        } else {
            todo.completed = true;
            Ok(())
        }
    } else {
        Err(format!("Task with ID {} not found!", id))
    }
}

fn delete_todo(list: &mut TodoList, id: usize) -> Result<(), String> {
    if let Some(pos) = list.todos.iter().position(|t| t.id == id) {
        list.todos.remove(pos);
        Ok(())
    } else {
        Err(format!("Task with ID {} not found!", id))
    }
}

fn save_to_file(list: &TodoList, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();
    content.push_str(&format!("{}\n", list.next_id));
    
    for todo in &list.todos {
        content.push_str(&format!(
            "{}|{}|{}|{}\n",
            todo.id,
            todo.description,
            todo.completed,
            todo.created_at
        ));
    }
    
    fs::write(path, content)?;
    Ok(())
}

fn load_from_file(path: &str) -> Result<TodoList, Box<dyn std::error::Error>> {
    match fs::read_to_string(path) {
        Ok(contents) => {
            let lines: Vec<&str> = contents.trim().split('\n').collect();
            
            if lines.is_empty() {
                return Ok(TodoList::new());
            }
            
            let next_id = lines[0].parse::<usize>().unwrap_or(1);
            let mut todos = Vec::new();
            
            for line in lines.iter().skip(1) {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 4 {
                    if let Ok(id) = parts[0].parse::<usize>() {
                        todos.push(Todo {
                            id,
                            description: parts[1].to_string(),
                            completed: parts[2] == "true",
                            created_at: parts[3].to_string(),
                        });
                    }
                }
            }
            
            Ok(TodoList { todos, next_id })
        }
        Err(_) => Ok(TodoList::new()),
    }
}

fn print_help() {
    println!("\nAvailable Commands:");
    println!("  add <description> - Add a new task");
    println!("  list             - List all tasks");
    println!("  done <id>        - Mark a task as completed");
    println!("  delete <id>      - Delete a task");
    println!("  help             - Show this help message");
    println!("  exit             - Exit the application");
    println!();
}

fn get_data_file_path() -> String {
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    
    let mut path = PathBuf::from(home_dir);
    path.push(".todos.txt");
    
    path.to_string_lossy().to_string()
}

fn get_current_time() -> String {
    // Simple timestamp without external dependencies
    "2024-01-20 12:00".to_string()
}