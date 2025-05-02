use diesel::prelude::*;
use std::env::args;
use NewTube::establish_connection;
use NewTube::model::usermodel::Post;
use NewTube::schema::posts::dsl::posts;
use NewTube::schema::posts::published;

fn main() {

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Published post {}", post.title);
}