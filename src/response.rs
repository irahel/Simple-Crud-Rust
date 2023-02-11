use crate::model::Todo;
use serde::Serialize;

#[derive(Serialize)]
pub struct response_json {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct todo_data {
    pub todo: Todo,
}

#[derive(Serialize, Debug)]
pub struct single_todo_response {
    pub status: String,
    pub data: TodoData,
}

#[derive(Serialize, Debug)]
pub struct todo_list_response {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}
