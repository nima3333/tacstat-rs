use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use anyhow::Result;

async fn init_db() -> Result<Pool<Sqlite>> {
   let pool = SqlitePool::connect("sqlite:tacview.db").await?;

   sqlx::query(
       "CREATE TABLE IF NOT EXISTS flights (
           flight_id INTEGER PRIMARY KEY AUTOINCREMENT,
           player_name TEXT NOT NULL,
           vehicle TEXT NOT NULL,
           time_in_game DOUBLE PRECISION NOT NULL,
           file_date TIMESTAMP NOT NULL
       )"
   ).execute(&pool).await?;

   sqlx::query(
       "CREATE TABLE IF NOT EXISTS weapons (
           weapon_id INTEGER PRIMARY KEY AUTOINCREMENT,
           weapon_name TEXT UNIQUE NOT NULL
       )"
   ).execute(&pool).await?;

   sqlx::query(
       "CREATE TABLE IF NOT EXISTS flight_weapons (
           flight_id INTEGER,
           weapon_id INTEGER,
           count INTEGER NOT NULL,
           PRIMARY KEY (flight_id, weapon_id),
           FOREIGN KEY (flight_id) REFERENCES flights(flight_id),
           FOREIGN KEY (weapon_id) REFERENCES weapons(weapon_id)
       )"
   ).execute(&pool).await?;

//    sqlx::query(
//        "CREATE INDEX IF NOT EXISTS idx_flights_creation_time ON flights(creation_time)"
//    ).execute(&pool).await?;

   Ok(pool)
}

#[tokio::main]
async fn main() -> Result<()> {
   let pool = init_db().await?;
   println!("Database initialized successfully!");
   Ok(())
}