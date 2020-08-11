use crate::persistence::database;
use std::error;

pub struct ProfessorConfig {
    database_uri: String,
}

impl ProfessorConfig {
    pub fn new(database_uri: String) -> ProfessorConfig {
        ProfessorConfig { database_uri }
    }
}

pub struct Professor {
    config: ProfessorConfig,
    database: database::Database,
}

impl Professor {
    pub fn new(config: ProfessorConfig) -> Professor {
        let database = database::Database::new(&config.database_uri);
        Professor { config, database }
    }
    pub async fn run(&self) -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }
}
