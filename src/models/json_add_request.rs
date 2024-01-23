#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct JsonAddRequest {
    title: String,
    task_completed: bool
}

impl JsonAddRequest {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_task_completed(&self) -> bool {
        self.task_completed
    }
}