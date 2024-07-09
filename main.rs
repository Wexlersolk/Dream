use dream::configuration::*;
use dream::routes::*;
use dream::startup::run;
use env_logger::Env;
use sqlx::PgPool;
use std::env;
use std::error::Error;

use dream::draw::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let meditation = &args[1];
    let days_to_show = &args[2];

    let file_path = "dream.wiki";
    let day_info = parse_file(file_path)?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.connection_string())
        .expect("Failed to connect to Postgres.");

    let mut paint = Paint {
        date: Vec::new(),
        rating: Vec::new(),
    };

    run(
        day_info,
        &mut paint,
        meditation.to_string(),
        days_to_show.to_string(),
        connection_pool,
    )
    .await?;
    Ok(())
}
