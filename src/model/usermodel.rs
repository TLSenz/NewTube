use serde::{Serialize,Deserialize};
use diesel::prelude::*;
use std::fmt::Display;
use rocket::response::Responder;

use crate::schema::posts;
use crate::schema::users;


#[derive(Deserialize, Debug, Clone)]
pub struct LoginInfo{
    pub username: String,
    pub password: String

}

#[derive(Serialize,Debug)]
pub struct LoginResponse{
    pub jwt_access_token: String,
}
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserOwned {
    pub username: String,
    pub password: String,
    pub email: String,
}


#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id:  i32,
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Claims{
    pub username: String,
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

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}