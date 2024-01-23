use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonUpdateRequest {
    title: Option<String>,
    has_completed: Option<bool>
}

impl JsonUpdateRequest {
    pub fn get_title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn get_has_completed(&self) -> Option<bool> {
        self.has_completed
    }
}