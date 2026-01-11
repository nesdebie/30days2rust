use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        println!("✓ Task added successfully!");
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("\nNo tasks yet. Add one to get started!");
            return;
        }

        println!("\n=== Your Tasks ===");
        for task in &self.tasks {
            let status = if task.completed { "✓" } else { " " };
            println!("[{}] {}: {}", status, task.id, task.description);
        }
        println!();
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            println!("✓ Task {} marked as completed!", id);
        } else {
            println!("✗ Task with ID {} not found.", id);
        }
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("✓ Task {} removed!", id);
        } else {
            println!("✗ Task with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    println!("=== Todo List CLI ===");
    println!("Manage your tasks efficiently!\n");

    loop {
        println!("Commands:");
        println!("  1. Add task");
        println!("  2. List tasks");
        println!("  3. Complete task");
        println!("  4. Remove task");
        println!("  5. Exit");

        let choice = read_input("\nEnter your choice: ").trim().to_string();

        match choice.as_str() {
            "1" => {
                let description = read_input("Enter task description: ").trim().to_string();
                if !description.is_empty() {
                    todo_list.add_task(description);
                } else {
                    println!("✗ Task description cannot be empty.");
                }
            }
            "2" => {
                todo_list.list_tasks();
            }
            "3" => {
                let id_str = read_input("Enter task ID to complete: ").trim().to_string();
                match id_str.parse::<usize>() {
                    Ok(id) => todo_list.complete_task(id),
                    Err(_) => println!("✗ Invalid ID. Please enter a number."),
                }
            }
            "4" => {
                let id_str = read_input("Enter task ID to remove: ").trim().to_string();
                match id_str.parse::<usize>() {
                    Ok(id) => todo_list.remove_task(id),
                    Err(_) => println!("✗ Invalid ID. Please enter a number."),
                }
            }
            "5" => {
                println!("Goodbye! 👋");
                break;
            }
            _ => {
                println!("✗ Invalid choice. Please select 1-5.");
            }
        }
        println!();
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

