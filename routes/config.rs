use chrono::NaiveDate;

pub struct DayInfo {
    pub day_name: String,
    pub date: NaiveDate,
    pub rating: u32,
    pub tasks: Vec<String>,
}
