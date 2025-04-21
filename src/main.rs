#[macro_use] extern crate rocket;
mod service;
mod repository;

use service::userservice::check_login;

use rocket::http::Status;

#[derive(serde::Deserialize)]
struct User {
    username: String,
    password: String,
    email: String
}


#[derive(serde::Deserialize, Debug)]
struct Login {
    username: String,
    password: String
}



#[post("/login", data= "<data>")]
async fn login(data: rocket::serde::json::Json<Login>)-> Status{
    println!("{:?}", data);
    check_login().await
}





#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login])
}