use diesel::prelude::*;
use std::env;
use diesel::associations::HasTable;
use dotenv::dotenv;

use crate::schema::posts;
use diesel::pg::PgConnection;
use crate::model::usermodel::{NewPost, NewUser, Post, User};
// für deinen eigenen Code

#[macro_use] extern crate rocket;


pub mod controller {
    pub mod usercontroller;
    pub mod videocontroller;
}

pub mod repository{
    pub mod userrepository;
}

pub mod service{
    pub mod userservice;
}

pub mod model{
    pub mod usermodel;
    pub mod videomodel;
}

pub mod security{
    pub mod authentication;
}

pub mod schema;


use crate::controller::usercontroller::{login, signup};
use crate::schema::users::dsl::users;
use crate::schema::users::password;

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![login,signup])
}
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}






pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}





