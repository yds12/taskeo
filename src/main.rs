use std::env;
use task::Task;

mod file;
mod fs;
mod task;

const DEFAULT_PRIO: u8 = 4;

/// List all tasks
fn list() {
    let tasks = file::get_tasks();

    match tasks {
        Ok(mut tasks) => {
            if tasks.len() == 0 {
                println!("Your TO-DO list is empty. Great!\n");
                return;
            }

            tasks.sort();

            println!("Here is your TO-DO list:\n");
            for (n, task) in tasks.iter().enumerate() {
                println!(" {:>3}. {}", n + 1, task);
            }
            println!("");
        }
        Err(e) => {
            println!("Error listing tasks: {e}");
        }
    }
}

/// Delete a task with given `id`. If you have more than 255 open tasks you
/// might have a problem bigger than an integer overflow.
fn delete(id: u8) {
    println!("Deleting task {}...", id);
    if let Err(e) = file::delete_task(id) {
        println!("Error deleting task: {e}");
    }
}

/// Add a new task with description `task` and a certain `priority` level.
fn add(task: &str, priority: u8) {
    println!("Adding task `{}` with priority {}...", task, priority);
    let task = Task::new(task.to_owned(), priority);
    if let Err(e) = file::add_task(task) {
        println!("Error adding task: {e}");
    }
}

fn is_prio(arg: &str) -> bool {
    ["%", "%%", "%%%", "%%%%"].contains(&arg)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match args.len() - 1 {
        0 => list(),
        1 if args[1].parse::<u8>().is_ok() => delete(args[1].parse::<u8>().unwrap()),
        1 => add(&args[1][..], DEFAULT_PRIO),
        _ => {
            if is_prio(&args[1]) {
                add(&args[2..].join(" "), args[1].len() as u8);
            } else {
                add(&args[1..].join(" "), DEFAULT_PRIO);
            }
        }
    }
}
