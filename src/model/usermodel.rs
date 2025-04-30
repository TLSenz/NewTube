use serde::{Serialize,Deserialize};
#[derive(Deserialize, Debug)]
pub struct LoginInfo{
    pub username: String,
    pub password: String

}

#[derive(Serialize)]
pub struct LoginResponse{
    pub jwt_access_token: String,
    pub jwt_refresh_token: String
}


#[derive(Deserialize, Serialize)]
pub struct User {
    pub id:  i32,
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Claims{
    pub subject_id: i32,
    pub(crate) exp: usize
}

pub struct JWT{
    pub claims: Claims
}


#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
