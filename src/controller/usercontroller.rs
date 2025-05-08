use std::result;
use diesel::{RunQueryDsl, SelectableHelper};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::establish_connection;
use crate::service::userservice::{check_login, sing_up_user};
use crate::model::usermodel::{LoginInfo, LoginResponse, NewUser, NewUserOwned, User};
use crate::schema::*;


#[post("/login", data = "<data>")]
pub async fn login(data: Json<LoginInfo>) -> Result<Json<LoginResponse>, Status> {
    let result = check_login(data.into_inner()).await;

    println!("Result Controller: {:?}", result);

    match result {
        Some(response) => Ok(Json(response)),
        None => Err(Status::InternalServerError),
    }
}

#[post("/signup", data = "<data>")]
pub async fn signup(data: Json<NewUser<'_>>) -> Status {
    // Clone Daten in eine 'static-kompatible Struktur
    let data_owned = NewUserOwned {
        username: data.username.to_string(),
        password: data.password.to_string(),
        email: data.email.to_string(),
    };

    // Führe Diesel-Aufruf in einem separaten Thread aus
    let result = rocket::tokio::task::spawn_blocking(move || {
        let mut conn = establish_connection();

        diesel::insert_into(users::table)
            .values(&data_owned)
            .get_result::<User>(&mut conn)
    }).await;

    // Ergebnis auswerten
    match result {
        Ok(Ok(_user)) => Status::Ok,
        Ok(Err(_diesel_err)) => Status::InternalServerError,
        Err(_join_err) => Status::InternalServerError,
    }
}
