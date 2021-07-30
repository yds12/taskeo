use anyhow::{Result, bail};
use crate::task::Task;
use crate::fs;

pub fn get_tasks() -> Result<Vec<Task>> {
  if !fs::file_exists() {
    return Ok(Vec::new());
  }

  let contents = fs::load()?;
  let tasks = decode(&contents[..])?;
  Ok(tasks)
}

pub fn add_task(task: Task) -> Result<()> {
  let mut tasks = Vec::new();

  if fs::file_exists() {
    let contents = fs::load()?;
    tasks = decode(&contents[..])?;
  }

  tasks.push(task);
  let encoded_tasks = encode(&tasks)?;
  fs::save(encoded_tasks)
}

/// Deletes a task at index `number`. The index starts at 1.
pub fn delete_task(number: u8) -> Result<()> {
  if fs::file_exists() {
    let contents = fs::load()?;
    let mut tasks = decode(&contents[..])?;

    if (number - 1) as usize >= tasks.len() {
      bail!("Task {} does not exist!", number);
    }

    tasks.remove((number - 1) as usize);
    let encoded_tasks = encode(&tasks)?;
    fs::save(encoded_tasks)
  } else {
    bail!("Task list is empty!");
  }
}

fn encode(task: &Vec<Task>) -> Result<Vec<u8>> {
  let bytes = bincode::serialize(task)?;
  Ok(bytes)
}

fn decode(content: &[u8]) -> Result<Vec<Task>> {
  let file = bincode::deserialize(&content)?;
  Ok(file)
}

