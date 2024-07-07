use dream::configuration::*;
use dream::routes::*;
use dream::startup::run;
use env_logger::Env;
use sqlx::PgPool;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "dream.wiki";
    let day_info = parse_file(file_path)?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.connection_string())
        .expect("Failed to connect to Postgres.");

    run(day_info, "Remember".to_string(), connection_pool).await?;
    Ok(())
}
