use dream::configuration::get_configuration;
use dream::routes::*;
use dream::startup::run;
use env_logger::Env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file_path = "dream.wiki";
    let day_info = parse_file(file_path)?;

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres.");

    run(day_info, "Remember", connection_pool);
}
