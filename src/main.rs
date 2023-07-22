#![feature(decl_macro)]
use rocket::{routes};
use rocket_auth::{Users};

mod api;
use api::api_handler::*;



#[rocket::main]
async fn main() {
    let users = Users::open_rusqlite("/tmp/mydb.db").expect("Failed to open database");

    let result = rocket::build()
        .mount("/", routes![signup, login, logout, status])
        .manage(users)
        .launch()
        .await;

    if let Err(e) = result {
        eprintln!("Failed to launch server: {}", e);
    }
}