pub mod database;
pub mod models;
mod web;
use futures::TryFutureExt;
use tokio::try_join;
use std::error::Error;
use crate::database::menu;
use actix_web::{App, HttpServer};
use crate::web::{echo, hello, manual_hello};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    menu::main_menu().await?;
    // Web server future


    // Menu future
    //let menu_future = tokio::spawn(async {
      //  menu::main_menu().await.map_err(|e| e.into()) // Convert to Box<dyn Error + Send + Sync>
    //});

    // Join futures
    //let (server_result, menu_result) = try_join!(server_future, menu_future)?;

    // Check results
    //server_result?;
    //menu_result?;

    Ok(())
}
