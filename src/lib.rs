use diesel::prelude::*;
use std::env;
use dotenv::dotenv;
use crate::model::usermodel::{NewPost, Post};
use crate::schema::posts;
use diesel::pg::PgConnection;


pub mod model{
    pub mod usermodel;
}
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {

}




pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}