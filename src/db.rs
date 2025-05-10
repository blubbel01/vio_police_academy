use crate::error::AppError;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn get_db_connection() -> Result<DatabaseConnection, AppError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    Ok(db)
}
