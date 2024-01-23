#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Todo {
    pub id: sqlx::types::Uuid,
    pub title: String,
    pub task_completed: bool,
    pub data_created: sqlx::types::time::Date,
}