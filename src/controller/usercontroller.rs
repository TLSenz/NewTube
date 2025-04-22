use rocket::http::Status;
use crate::service::userservice;
use crate::service::userservice::check_login;
use crate::model::usermodel::LoginInfo;










#[post("/login", data= "<data>")]
pub async fn login(data: rocket::serde::json::Json<LoginInfo>) -> Status{
    println!("{:?}", data);
    check_login().await
}
