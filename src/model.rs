use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct to_do {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

//Data structure for the application state
pub struct app_state {
    pub todo_db: Arc<Mutex<Vec<to_do>>>,
}

impl app_state {
    pub fn init() -> app_state {
        app_state {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct update_todo_schema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
