use crate::models::Task;
use std::fs;
use std::path::Path;

pub fn load_tasks(file_path: &str) -> std::io::Result<Vec<Task>> {
    if Path::new(file_path).exists() {
        let data = fs::read_to_string(file_path)?;
        let tasks = serde_json::from_str(&data)?;
        Ok(tasks)
    } else {
        // Return an empty vector if the file does not exist
        Ok(Vec::new())
    }
}

pub fn save_tasks(tasks: &Vec<Task>, file_path: &str) -> std::io::Result<()> {
    let data = serde_json::to_string(tasks)?;
    fs::write(file_path, data)?;
    Ok(())
}