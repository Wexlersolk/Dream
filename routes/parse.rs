use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::routes::config::{DayInfo, Task};

pub fn parse_file(file_path: &str) -> io::Result<DayInfo> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    // Extract the day name
    let day_name = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing day name"))??
        .replace("==", "")
        .trim()
        .to_string();

    // Extract the date
    let date = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing date"))??
        .trim()
        .to_string();

    // Extract the rating line
    let rating_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing rating line"))??;

    // Parse the rating
    let rating = rating_line
        .replace("+Rating: *", "")
        .replace("*", "")
        .trim()
        .parse::<i32>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse rating: {}", e)))?;

    // Initialize the tasks vector
    let mut tasks = Vec::new();

    // Process remaining lines for tasks
 for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        // Store the entire line in the task description
        let description = line.trim().to_string();

        // Determine if the task is completed based on its content
        let completed = if description.contains("[X]") {
            true
        } else if description.contains("[ ]") {
            false
        } else {
            // Default to false if the status cannot be determined
            false
        };        tasks.push(Task {
            description: description.to_string(),
            completed,
        });
    }

    Ok(DayInfo {
        day_name,
        date,
        rating,
        tasks,
    })
}

