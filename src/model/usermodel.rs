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
struct User {
    username: String,
    password: String,
    email: String
}

