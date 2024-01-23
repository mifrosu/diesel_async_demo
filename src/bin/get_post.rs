use diesel_async_demo::*;
use self::models::Post;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::env::args;

// cargo run --bin get_post n
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");



    let url = db_url();
    if let Ok(connection) = &mut establish_connection(&url).await {
        let post = posts
            .find(post_id)
            .select(Post::as_select())
            .first(connection)
            .await
            .optional(); // optional() is required to handle the case where the post is not found 

        match post {
            Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
            Ok(None) => println!("Unable to find post {}", post_id),
            Err(_) => println!("An error occurred when fetching post {}", post_id),
        }


    }

    Ok(())
}