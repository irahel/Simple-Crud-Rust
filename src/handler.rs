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

//Create
#[post("/todos", data = "<body>")]
pub async fn create_todo_handler(
    mut body: Json<to_do>,
    data: &State<app_state>,
) -> Result<Json<single_todo_response>, Custom<Json<response>>> {
    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter() {
        if todo.title == body.title {
            let error_response = response {
                status: "fail".to_string(),
                message: format!("Todo with title: '{}' already exists", todo.title),
            };
            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body.into_inner());

    let json_response = single_todo_response {
        status: "success".to_string(),
        data: TodoData {
            todo: todo.into_inner(),
        },
    };

    Ok(Json(json_response))
}

//Read one
#[get("/todos/<id>")]
pub async fn get_todo_handler(
    id: String,
    data: &State<app_state>,
) -> Result<Json<single_todo_response>, Custom<Json<response>>> {
    let vec = data.todo_db.lock().unwrap();

    for todo in vec.iter() {
        if todo.id == Some(id.to_owned()) {
            let json_response = single_todo_response {
                status: "success".to_string(),
                data: todo_data { todo: todo.clone() },
            };

            return Ok(Json(json_response));
        }
    }

    let error_response = response {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}

#[patch("/todos/<id>", data = "<body>")]
pub async fn edit_todo_handler(
    id: String,
    body: Json<update_todo_schema>,
    data: &State<app_state>,
) -> Result<Json<single_todo_response>, Custom<Json<response>>> {
    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter_mut() {
        if todo.id == Some(id.clone()) {
            let datetime = Utc::now();
            let title = body.title.to_owned().unwrap_or(todo.title.to_owned());
            let content = body.content.to_owned().unwrap_or(todo.content.to_owned());
            let payload = Todo {
                id: todo.id.to_owned(),
                title: if !title.is_empty() {
                    title
                } else {
                    todo.title.to_owned()
                },
                content: if !content.is_empty() {
                    content
                } else {
                    todo.content.to_owned()
                },
                completed: if body.completed.is_some() {
                    body.completed
                } else {
                    todo.completed
                },
                createdAt: todo.createdAt,
                updatedAt: Some(datetime),
            };
            *todo = payload;

            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo: todo.clone() },
            };
            return Ok(Json(json_response));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };

    Err(Custom(Status::NotFound, Json(error_response)))
}
