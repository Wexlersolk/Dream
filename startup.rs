use crate::draw::*;
use crate::meditations::*;
use crate::remember::remember;
use crate::routes::config::DayInfo;
use sqlx::PgPool;
use std::error::Error;

pub async fn run(
    day_info: DayInfo,
    paint: &mut Paint,
    meditation: String,
    days_to_show: String,
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
            }
            Meditations::Create => {
                println!("Running Create meditation");
                draw(&day_info, 10, paint, &db_pool).await?;
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
