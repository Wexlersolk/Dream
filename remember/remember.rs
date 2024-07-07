use crate::routes::DayInfo;
use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn remember(day_info: &DayInfo, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
        r#"
        INSERT INTO dreams (score, tasks, date)
        VALUES ($1, $2, $3)
        ON CONFLICT (date) DO UPDATE
        SET score = EXCLUDED.score, tasks = EXCLUDED.tasks
        "#,
        day_info.rating,
        &day_info.tasks as &[String],
        day_info.date
    )
    .execute(pool)
    .await?;
    Ok(())
}

