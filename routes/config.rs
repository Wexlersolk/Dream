pub struct Task {
    pub description: String,
    pub completed: bool,
}

pub struct DayInfo {
    pub day_name: String,
    pub date: String,
    pub rating: i32,
    pub tasks: Vec<Task>,
}
