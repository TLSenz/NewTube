use std::result;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::service::userservice::check_login;
use crate::model::usermodel::{LoginInfo, LoginResponse};









#[post("/login", data = "<data>")]
pub async fn login(data: Json<LoginInfo>) -> Result<Json<LoginResponse>, Status> {
    let result = check_login(data.into_inner()).await;

    match result {
        Some(response) => Ok(Json(response)),
        None => Err(Status::Unauthorized),
    }
}
