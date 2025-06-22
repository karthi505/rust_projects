use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Task {
    task_name: String,
    status: bool,
}

fn mark_thing_done(path: &Path) -> std::io::Result<()> {
    let mut file_open = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    let reader = BufReader::new(&file_open);
    let mut view_task: Vec<Task> = serde_json::from_reader(reader).unwrap_or_default();

    for (i, task) in view_task.iter().enumerate() {
        println!("{}. {} -> [{}]", i + 1, task.task_name.trim(), task.status);
    }

    println!("Please Enter the Number of the Task to change!: ");
    let mut status_input = String::new();
    io::stdin().read_line(&mut status_input)?;

    let selected_index: usize = status_input.trim().parse().expect("Invalid input");

    if let Some(task) = view_task.get_mut(selected_index - 1) {
        task.status = true;
    } else {
        println!("Invalid task number.");
        return Ok(());
    }

    file_open.set_len(0)?;
    file_open.seek(SeekFrom::Start(0))?;
    let mut writer = BufWriter::new(&mut file_open);
    serde_json::to_writer_pretty(&mut writer, &view_task)?;
    writer.flush()?;

    Ok(())
}


fn write_down_tasks(path: &Path) -> std::io::Result<()> {
    
    let mut file = File::open(path).unwrap_or_else(|_| File::create(path).unwrap());//alternate way for OpenOption(which i prefer better than this)
    let reader = BufReader::new(file);

    let mut tasks: Vec<Task> = serde_json::from_reader(reader).unwrap_or_default();

    let mut description = String::new();
    println!("Enter your task:");
    std::io::stdin().read_line(&mut description).unwrap();

    let new_task = Task {
        task_name: description.trim().to_string(),
        status: false,
    };
    tasks.push(new_task);


    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks)?;

    Ok(())
}

fn delete_task(path : &Path) {

}

fn read_tasks(path : &Path) -> io::Result<()> {
    let file_open = File::open(path)?;
    let reader = BufReader::new(file_open);
    let mut view_task: Vec<Task> = serde_json::from_reader(reader).unwrap_or_default();

    println!("The Current Tasks -> From the File 'to_do_list' ");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++");

    for (i, task) in view_task.iter().enumerate() {
        println!("{}. {} -> [{}]", i + 1, task.task_name.trim(), task.status);
    }

    Ok(())
}
pub fn files() {

    let path : &Path = Path::new("to_do_list");
    
    println!("1. Add a New Task");
    println!("2. Mark the Task As Done(true)");
    println!("3. Read The Tasks");

    println!("Please Enter You're Input:(1/2/3)");
    let mut input : String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to Read the Input");

    match input.trim().chars().next(){
        Some('1') => {
            match write_down_tasks(&path) {
                Ok(file) => println!("Wrote down the task successfully"),
                Err(err) => println!("Something went wrong while writing down a task!"),
            }
        },

        Some('2') => {
            match mark_thing_done(&path) {
                Ok(file) => println!("Status Successfully Updated!"),
                Err(err) => println!("Fialed to Update Status"),
            }
        },

        Some('3') => {
            read_tasks(&path);
        },

        _ => {println!("Please input a valid Option(1/2");},
    }


}