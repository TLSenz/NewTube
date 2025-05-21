
use rocket::http::Status;
use rocket::Request;
use serde::__private::ser::FlatMapSerializeTupleVariantAsMapValue;
use crate::establish_connection;
use crate::model::usermodel::{LoginInfo, LoginResponse, NewUser};
use crate::repository::userrepository::{check_user, create_user, get_user};
use crate::security::authentication::create_jwt;
use crate::model::usermodel::User;
use crate::model::videomodel::UploadRequest;

pub async fn check_login(login_data: LoginInfo) -> Option<LoginResponse> {
    let result =
        check_user(login_data.clone()).await;

        println!("Result: {}", result);
        if result{
            println!("HEllo world");
            let token = create_jwt(login_data.username);
            match token {
                Ok(token) =>{
                    println!("token");
                    Some(LoginResponse{ jwt_access_token: token.to_string() })
                },
                Err(Error) => {
                    println!("Error: {}", Error);
                   None
                }
            }
        }
        else {
            None
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



pub async fn insert_video(request: UploadRequest){

}