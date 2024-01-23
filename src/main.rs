mod controllers;
mod models;
mod test;

use std::{sync::Arc, env};

use actix_web::{web::Data, App, HttpServer};
use controllers::api_controller::{get_todos, add_todos, delete_todo, update_todo, revert_todo, complete_todo};
use models::app_data::AppData;
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    let app_data = Arc::new(AppData::new().await);
    HttpServer::new(move || {
        App::new()
            .service(get_todos)
            .service(add_todos)
            .service(delete_todo)
            .service(update_todo)
            .service(revert_todo)
            .service(complete_todo)
            .app_data(Data::new(app_data.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
