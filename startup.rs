use crate::create::create;
use crate::meditations::*;
use crate::remember::remember;
use crate::routes::config::DayInfo;
use sqlx::PgPool;
use std::error::Error;

pub async fn run(
    day_info: DayInfo,
    meditation: String,
    db_pool: PgPool,
) -> Result<(), Box<dyn Error>> {
    match parse_meditation(&meditation) {
        Ok(option) => match option {
            Meditations::Remember => {
                println!("Running Remember meditation");
                remember(&day_info, &db_pool).await?;
            }
            Meditations::Recall => {
                println!("Running Recall meditation");
                // the recall function and call it here
            }
            Meditations::Create => {
                println!("Running Create meditation");
                create()?;
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
