use crate::routes::DayInfo;
use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn remember(day_info: &DayInfo, pool: &PgPool) -> Result<(), Error> {
    sqlx::query!(
        r#"
        UPDATE dreams
        SET score = $1, tasks = $2
        WHERE date = $3
        "#,
        day_info.rating,
        &day_info.tasks as &[String],
        day_info.date
    )
    .execute(pool)
    .await?;
    Ok(())
}

