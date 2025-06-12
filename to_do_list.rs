use std::io;

#[derive(Debug)]
struct Task {
    description: String,
    status: String,
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Enter the task description:");
    io::stdin().read_line(&mut description).expect("Failed to read input");

    let task = Task {
        description: description.trim().to_string(),
        status: String::from("Not yet Checked"),
    };

    tasks.push(task);
    println!("Task added successfully!\n");
}

fn view_task(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.\n");
        return;
    }

    println!("\n===== Your Tasks =====");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, task.description, task.status);
    }
    println!("======================\n");
}

fn confirm_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to mark as done.\n");
        return;
    }

    view_task(tasks);

    println!("Enter the task number to mark as done:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if let Ok(index) = input.trim().parse::<usize>() {
        if index == 0 || index > tasks.len() {
            println!("Invalid task number.\n");
        } else {
            tasks[index - 1].status = String::from("Checked");
            println!("Task marked as done!\n");
        }
    } else {
        println!("Please enter a valid number.\n");
    }
}

pub fn do_list() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut cont = true;

    while cont {
        println!("1. Add A Task");
        println!("2. View the Task");
        println!("3. Check That Task off");
        println!("4. Done!");

        let mut input = String::new();
        println!("Enter the Choice (1/2/3/4): ");
        io::stdin().read_line(&mut input).expect("Reading Failed");

        match input.trim().chars().next() {
            Some('1') => add_task(&mut tasks),
            Some('2') => view_task(&tasks),
            Some('3') => confirm_task(&mut tasks),
            Some('4') => cont = false,
            _ => println!("Invalid input. Please enter 1, 2, 3, or 4.\n"),
        }
    }

    println!("Thank you for using the to-do list!");
}
