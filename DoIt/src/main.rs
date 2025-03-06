use serde::{Deserialize, Serialize};
use serde_json;
fn main() {
    let file = std::fs::read_to_string("todo_list.json").unwrap();
    let _data: serde_json::Value = serde_json::from_str(&file).unwrap();// Define the TodoItem structure

    // Convert JSON to Vec<TodoItem>
    let mut todos: Vec<TodoItem> = serde_json::from_str(&file).unwrap();
    loop {
        println!("\n--- Todo List ---");
        for (i, item) in todos.iter().enumerate() {
            println!("{}. [{}] {}",
                i + 1,
                if item.completed { "x" } else { " " },
                item.title
            );
        }
        println!("\nCommands:");
        println!("1. Add todo");
        println!("2. Toggle todo");
        println!("3. Delete todo");
        println!("4. Save and exit");
        let input = get_input("Enter command:");
        match input.trim() {
            "1" => {
                let title = get_input("Enter todo title:");
                todos.push(TodoItem {
                    title: title.trim().to_string(),
                    completed: false,
                });
            },
            "2" => {
                let num = get_input("Enter todo number to toggle:");
                if let Ok(i) = num.trim().parse::<usize>() {
                    if i > 0 && i <= todos.len() {
                        todos[i-1].completed = !todos[i-1].completed;
                    }
                }
            },
            "3" => {
                let num = get_input("Enter todo number to delete:");
                if let Ok(i) = num.trim().parse::<usize>() {
                    if i > 0 && i <= todos.len() {
                        todos.remove(i-1);
                    }
                }
            },
            "4" => {
                let json = serde_json::to_string_pretty(&todos).unwrap();
                std::fs::write("todo_list.json", json).unwrap();
                break;
            },
            _ => println!("Invalid command"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    title: String,
    completed: bool,
}
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
