use std::result;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::service::userservice::{check_login, sing_up_user};
use crate::model::usermodel::{LoginInfo, LoginResponse, NewUser};









#[post("/login", data = "<data>")]
pub async fn login(data: Json<LoginInfo>) -> Result<Json<LoginResponse>, Status> {
    let result = check_login(data.into_inner()).await;

    match result {
        Some(response) => Ok(Json(response)),
        None => Err(Status::InternalServerError),
    }
}

#[post("/signup", data="<data>")]
pub async  fn signup(data: Json<NewUser<'_>>)-> Status{
    let result = sing_up_user(data.into_inner()).await;

    Status::Ok

}
