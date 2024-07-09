use crate::draw::*;
use crate::routes::DayInfo;
use sqlx::postgres::PgPool;
use std::error::Error;

pub async fn draw(
    day_info: &DayInfo,
    days_to_show: i32,
    paint: &mut Paint,
    pool: &PgPool,
) -> Result<(), Box<dyn Error>> {
    get_paint(&day_info, days_to_show, paint, pool).await?;
    Ok(())
}
