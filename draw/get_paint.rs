use crate::draw::Paint;
use crate::routes::DayInfo;
use chrono::NaiveDate;
use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn get_paint(
    day_info: &DayInfo,
    days_to_show: i32,
    paint: &mut Paint,
    pool: &PgPool,
) -> Result<(), Error> {
    let end_date = day_info.date;

    for i in 0..days_to_show {
        let current_date = end_date - chrono::Duration::days(i as i64);

        let record = sqlx::query!(
            r#"
            SELECT date, score FROM dreams
            WHERE date = $1
            "#,
            current_date
        )
        .fetch_optional(pool)
        .await?;

        if let Some(record) = record {
            paint.date.push(current_date);

            if let Some(score) = record.score {
                paint.rating.push(score as u32);
            }
        }
    }

    println!("Dates and Ratings:");
    for (date, rating) in paint.date.iter().zip(&paint.rating) {
        println!("Date: {}, Rating: {}", date, rating);
    }
    Ok(())
}
