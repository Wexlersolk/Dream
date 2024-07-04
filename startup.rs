use crate::meditations::*;
use crate::remember::remember;
use crate::routes::*;
use sqlx::PgPool;

pub fn run(day_info: &DayInfo, meditation: String, db_pool: PgPool) {
    match parse_meditation(&meditation) {
        Ok(option) => match option {
            Meditations::Remember => {
                println!("Running Remember meditation");
                remember(&day_info);
            }
            Meditations::Recall => {
                println!("Running Recall meditation");
                //      recall(&db_pool);
            }
            Meditations::Create => {
                println!("Running Create meditation");
                //     create(&db_pool);
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
