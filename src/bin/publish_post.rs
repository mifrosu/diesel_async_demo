use diesel_async_demo::*;
use self::models::Post;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::env::args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish post requires an ID")
        .parse::<i32>()
        .expect("Invalid ID");

    let url = db_url();
    if let Ok(connection) = &mut establish_connection(&url).await {
        let post = diesel::update(posts.find(id))
            .set(published.eq(true))
            .returning(Post::as_returning())
            .get_result(connection)
            .await;
        match post {
            Ok(post) => println!("Published post {}", post.title),
            Err(e) => println!("Unable to publish post {}", e.to_string()),
       }
    }

    Ok(())
}
