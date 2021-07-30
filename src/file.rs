use anyhow::Result;
use crate::task::Task;
use crate::fs;

pub fn get_tasks() -> Result<Vec<Task>> {
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

fn encode(task: &Vec<Task>) -> Result<Vec<u8>> {
  let bytes = bincode::serialize(task)?;
  Ok(bytes)
}

fn decode(content: &[u8]) -> Result<Vec<Task>> {
  let file = bincode::deserialize(&content)?;
  Ok(file)
}

