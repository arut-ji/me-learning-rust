use diesel::prelude::*;
use std::env::args;
use diesel_demo::models::Post;
use diesel_demo::establish_connection;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};
    let id = args()
        .nth(1).expect("published_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post{}", id));
}
