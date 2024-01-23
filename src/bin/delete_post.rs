use diesel_async_demo::*;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::env::args;

/// cargo run --bin delete_post <title match>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let url = db_url();

    if let Ok(connection) = &mut establish_connection(&url).await {
        let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
            .execute(connection)
            .await;
        match num_deleted {
            Ok(num_deleted) => println!("Deleted {} posts", num_deleted),
            Err(e) => println!("Error deleting posts {}", e.to_string()),
        }
    }

    Ok(())
}