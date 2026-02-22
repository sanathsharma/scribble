use crate::constants::{DATABASE_DIR, DATABASE_NAME};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use std::path;

pub async fn create_db_pool() -> Result<SqlitePool, sqlx::Error> {
  let database_dir = shellexpand::tilde(DATABASE_DIR).to_string();
  let pathbuf = path::PathBuf::from(database_dir.clone());
  if !pathbuf.exists() {
    std::fs::create_dir_all(&pathbuf)?;
  }

  let url = format!("sqlite://{}/{}", database_dir.as_str(), DATABASE_NAME);
  let options = SqliteConnectOptions::from_str(&url)
    .unwrap()
    .create_if_missing(true);

  let pool = SqlitePool::connect_with(options).await?;
  sqlx::migrate!().run(&pool).await?;

  Ok(pool)
}
