use crate::routes::*;

pub fn remember(day_info: &DayInfo) {
    println!("Day: {}", day_info.day_name);
    println!("Date: {}", day_info.date);
    println!("Rating: {}", day_info.rating);
    println!("Tasks:");
    for task in &day_info.tasks {
        println!("{}", task.description);
    }
}
