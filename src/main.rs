use dotenv::dotenv;
use professor::professor::{Professor, ProfessorConfig};
use std::env;
use std::error::Error;
use std::process;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem initializing professor config: {}", err);
        process::exit(1);
    });
    let professor = Professor::new(config).unwrap_or_else(|err| {
        eprintln!("Problem initializing professor: {}", err);
        process::exit(1);
    });

    planner.run().await.unwrap_or_else(|err| {
        eprintln!("Problem running the keeper: {}", err);
        process::exit(1);
    });
}

fn get_config() -> Result<PlannerConfig, Box<dyn Error>> {
    let nats_publisher_uri = env::var("NATS_URI")?;
    let nats_publisher_uri = Url::parse(&nats_publisher_uri)?;

    let nats_publisher_subject = String::from("url");

    let database_uri = env::var("DATABASE_URI")?;
    let database_uri = Url::parse(&database_uri)?;

    let starting_url = env::var("STARTING_URL")?;
    let starting_url = Url::parse(&starting_url)?;

    let config = PlannerConfig::new(
        nats_publisher_uri.into_string(),
        nats_publisher_subject,
        database_uri.into_string(),
        starting_url.into_string(),
    );
    Ok(config)
}
