use crate::error::AppError;
use sea_orm::{Database, DatabaseConnection};

pub async fn get_db_connection(database_url: &str) -> Result<DatabaseConnection, AppError> {
    let db = Database::connect(database_url).await?;
    Ok(db)
}
