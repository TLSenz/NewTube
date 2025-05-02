use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use NewTube::establish_connection;
use NewTube::model::usermodel::Post;
use NewTube::schema::posts::dsl::posts;
use NewTube::schema::posts::published;
use diesel::ExpressionMethods;

fn main() {


    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}