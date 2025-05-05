
use rocket::http::Status;
use serde::__private::ser::FlatMapSerializeTupleVariantAsMapValue;
use crate::establish_connection;
use crate::model::usermodel::{LoginInfo, LoginResponse, NewUser};
use crate::repository::userrepository::{check_user, create_user, get_user};
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

pub async fn sing_up_user(user: NewUser<'_>) -> User{

    let username_owned = user.username.to_string();
    let password_owned = user.password.to_string();
    let email_owned = user.email.to_string();

    let result = rocket::tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();
        create_user(
            &mut conn,
            &username_owned,
            &password_owned,
            &email_owned
        )
    }).await;


    result.unwrap()

}