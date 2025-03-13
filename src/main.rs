mod api;
mod database;

use crate::api::v1::routes::*;
use crate::database::postgres;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let db_pool = postgres::connect();

    

    Ok(())
}