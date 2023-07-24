#![feature(decl_macro)]
use rocket::{routes};
use rocket_auth::{Users};

mod api;
use api::api_handler::*;

use rocket::serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    database_path: String,
    rocket_mountpoint: String,
}

#[rocket::main]
async fn main() {
    let figment = rocket::Config::figment();
    let config: AppConfig = figment.extract().expect("Failed to extract config");

    let users = Users::open_rusqlite(&config.database_path).expect("Failed to open database");

    let result = rocket::build()
        .mount(&config.rocket_mountpoint, routes![signup, login, logout, status])
        .manage(users)
        .launch()
        .await;

    if let Err(e) = result {
        eprintln!("Failed to launch server: {}", e);
    }
}