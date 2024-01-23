use sqlx::Executor;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::Row;

use sqlx::postgres::PgQueryResult;
use sqlx::postgres::PgRow;
use uuid::Uuid;
use crate::models::db_controller::DbController;
use crate::models::todo::Todo;
impl DbController {

    pub async fn connect(db_connection_string: &str) -> DbController {
        // match PgPool::connect(db_connection_string).await {
        //     Ok(pool) => {
        //         DbController {
        //             connection: Some(pool)
        //         }
        //     }
        //     Err(e) => {
        //         DbController {
        //             connection: None
        //         }
        //     }
        // }
        DbController{connection: Some(PgPool::connect(db_connection_string).await.unwrap()) }
    }

    pub async fn get_todos(&self) -> Vec<Todo> {
        // match &self.connection {
        //     Some(conn) => {
        //         let mut connection = conn.acquire().await.unwrap();
        //         match sqlx::query::<Postgres>("SELECT * FROM todos").fetch_all(&mut *connection).await {
        //             Ok(x) => {
        //                 x.iter().map(|row: &PgRow| -> Todo {
        //                     Todo {
        //                         id: row.get(0),
        //                         title: row.get(1),
        //                         task_completed: row.get(2),
        //                         data_created: row.get(3),
        //                     }
        //                 }).collect::<Vec<Todo>>()
        //             },
        //             Err(_) => {
        //                 log::error!("Unable to fetch a Postgres connection from pool");
        //                 vec![]
        //             }
        //         }
        //     },
        //     None => {
        //         log::error!("Pg Connection not set. Please call DbController.connect() somewheres.");
        //         vec![]
        //     },
        // }


        let conn = &self.connection.clone().unwrap();

        let mut connection = conn.acquire().await.unwrap();

        let result = sqlx::query::<Postgres>("SELECT * FROM todos").fetch_all(&mut *connection).await.unwrap();

        result.iter().map(|row: &PgRow| -> Todo {
                                Todo {
                                    id: row.get(0),
                                    title: row.get(1),
                                    task_completed: row.get(2),
                                    data_created: row.get(3),
                                }
                            }).collect::<Vec<Todo>>()
    }

    pub async fn add_todo(&self, title: String, task_completed: bool) -> Result<PgQueryResult, sqlx::Error>{
        self.connection.clone().unwrap().acquire().await.unwrap()
        .execute(sqlx::query!("INSERT INTO todos (title, task_completed) VALUES ($1, $2)", title, task_completed)).await
    }

    pub async fn update_todo(&self, uuid: Uuid, title: Option<String>, task_completed: Option<bool>) {
        let mut conn = self.connection.clone().unwrap().acquire().await.unwrap();
        match title {
            Some(title) => {
                match task_completed {
                    Some(completed) => {
                        conn.execute(sqlx::query!("UPDATE todos SET title = $1, task_completed = $2 WHERE id = $3", title, completed, uuid)).await.unwrap();
                    },
                    None => {
                        conn.execute(sqlx::query!("UPDATE todos SET title = $1 WHERE id = $2", title, uuid)).await.unwrap();
                    }
                }
            },
            None => {
                match task_completed {
                    Some(completed) => {
                        conn.execute(sqlx::query!("UPDATE todos SET task_completed = $1 WHERE id = $2", completed, uuid)).await.unwrap();
                    },
                    None => {
                        return;
                    }
                }
            }
        }
        
    }

    pub async fn delete_todo (&self, uuid: Uuid) {
        let mut conn = self.connection.clone().unwrap().acquire().await.unwrap();
        conn.execute(sqlx::query!("DELETE FROM todos WHERE id = $1", uuid)).await.unwrap();
    }

    pub async fn complete_todo (&self, uuid: Uuid) {
        let mut conn = self.connection.clone().unwrap().acquire().await.unwrap();
        conn.execute(sqlx::query!("UPDATE todos SET task_completed = $1 WHERE ID = $2", true, uuid)).await.unwrap();
    }

    pub async fn revert_todo (&self, uuid: Uuid) {
        let mut conn = self.connection.clone().unwrap().acquire().await.unwrap();
        conn.execute(sqlx::query!("UPDATE todos SET task_completed = $1 WHERE ID = $2", false, uuid)).await.unwrap();
    }
}