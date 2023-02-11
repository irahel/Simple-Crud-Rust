use crate::{
    model::{app_state, to_do, update_todo_schema},
    response::{response, single_todo_response, todo_data, todo_list_response},
};
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State,
};
use uuid::Uuid;

#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<response>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = response {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

//Read all
#[get("/todos?<page>&<limit>")]
pub async fn todos_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<app_state>,
) -> Result<Json<todo_list_response>, Status> {
    let vec = data.todo_db.lock().unwrap();

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let todos: Vec<to_do> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = todo_list_response {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };
    Ok(Json(json_response))
}
