use crate::{
    model::{app_state, to_do, update_todo_schema},
    response::{response, single_todo_response, todo_data, todo_list_response},
};
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State,
};
use uuid::Uuid;
