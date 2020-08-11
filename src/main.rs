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
    let professor = Professor::new(config);

    professor.run().await.unwrap_or_else(|err| {
        eprintln!("Problem running professor: {}", err);
        process::exit(1);
    });
}

fn get_config() -> Result<ProfessorConfig, Box<dyn Error>> {
    let database_uri = env::var("DATABASE_URI")?;
    let database_uri = Url::parse(&database_uri)?;

    let config = ProfessorConfig::new(database_uri.into_string());
    Ok(config)
}
