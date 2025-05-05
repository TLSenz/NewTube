
use rocket::http::Status;
use serde::__private::ser::FlatMapSerializeTupleVariantAsMapValue;
use crate::model::usermodel::{LoginInfo, LoginResponse};
use crate::repository::userrepository::{check_user, get_user};
use crate::security::authentication::create_jwt;
use crate::model::usermodel::User;

pub async fn check_login(login_data: LoginInfo, ) -> Option<LoginResponse>{

    let result = check_user(login_data).await;

    match result{
        Ok(user) => {
           let jwt_token  = create_jwt(user.id);
            match jwt_token {
                Ok(token) => {
                    let response:LoginResponse = LoginResponse{ jwt_access_token:  token};
                    Option::from(response)
                }
                Err(E) => {
                    None
                }
            }
        }
        _ => {
            None
        }
    }


}