
#[cfg(test)]
mod test {
    use crate::models::db_controller::DbController;
    #[actix_web::test]
    async fn test_get_todos() {
        let controller = DbController::connect("postgresql://rick:696969@localhost/todos").await;
        controller.get_todos().await.iter().for_each(|x| {
            println!("{:#?}", x);
        });
    }
}