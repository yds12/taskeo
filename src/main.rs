use std::env;
use regex::Regex;
use task::Task;

mod task;
mod fs;
mod file;

const DEFAULT_PRIO: u8 = 4;

/// List all tasks
fn list() {
  let tasks = file::get_tasks();

  if let Ok(mut tasks) = tasks {
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
  } else {
    println!("Error listing tasks.");
  }
}

/// Delete a task with given `id`. If you have more than 255 open tasks you
/// might have a problem bigger than an integer overflow.
fn delete(id: u8) {
  println!("Deleting task {}...", id);
  if let Err(_) = file::delete_task(id) {
    println!("Error deleting task.");
  }
}

/// Add a new task with description `task` and a certain `priority` level.
fn add(task: &str, priority: u8) {
  println!("Adding task `{}` with priority {}...", task, priority);
  let task = Task::new(task.to_owned(), priority);
  if let Err(_) = file::add_task(task) {
    println!("Error adding task.");
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() - 1 {
    0 => list(),
    1 if args[1].parse::<u8>().is_ok() => delete(args[1].parse::<u8>().unwrap()),
    1 => add(&args[1][..], DEFAULT_PRIO),
    _ => {
      let r = Regex::new(r"(%)+").unwrap();
      if r.is_match(&args[1]) {
        add(&args[2..].join(" "), args[1].len() as u8);
      } else {
        add(&args[1..].join(" "), DEFAULT_PRIO);
      }
    }
  }
}

