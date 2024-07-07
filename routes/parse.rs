use crate::routes::config::DayInfo;
use chrono::NaiveDate;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_file(file_path: &str) -> io::Result<DayInfo> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let day_name = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing day name"))??
        .replace("==", "")
        .trim()
        .to_string();

    let date_str = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing date"))??
        .trim()
        .to_string();

    println!("Date string: {}", date_str); // Corrected println! statement

    // Parse the date string into a NaiveDate
    let date = NaiveDate::parse_from_str(&date_str, "%d.%m.%Y").map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse date: {}", e),
        )
    })?;

    let rating_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing rating line"))??;

    let rating = rating_line
        .replace("+Rating: *", "")
        .replace("*", "")
        .trim()
        .parse::<i32>()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse rating: {}", e),
            )
        })?;

    let mut tasks = Vec::new();

    for line in lines {
        let line = line?;
        if !line.trim().is_empty() {
            tasks.push(line.trim().to_string());
        }
    }

    Ok(DayInfo {
        day_name,
        date,
        rating,
        tasks,
    })
}
