use super::db_controller::DbController;

#[derive(Debug)]
pub struct AppData {
    controller: super::db_controller::DbController
}

impl AppData {
    pub fn get_db_controller(&self) -> &DbController{
        return &self.controller;
    }

    pub async fn new() -> AppData{
        let conn = DbController::connect(&std::env::var("DATABASE_URL").expect("databse url not found")).await;
        AppData {
            controller: conn
        }
    }
}