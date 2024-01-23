use self::models::*;
use diesel::prelude::*;
use diesel_async_demo::*;
use diesel_async::RunQueryDsl;

// cargo run --bin show_posts
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The use self::schema::posts::dsl::* line imports a bunch of aliases 
    // so that we can say posts instead of posts::table, and published 
    // instead of posts::published. It’s useful when we’re only dealing with
    //  a single table, but that’s not always what we want.
    //  It’s always important to keep imports to schema::table::dsl::* inside 
    // of the current function to prevent polluting the module namespace.
    use self::schema::posts::dsl::*;

    let url = db_url();

    if let Ok(connection) = &mut establish_connection(&url).await {
        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .await;

        match results {
            Ok(results) => {
                println!("Displaying {} posts", results.len());
                for post in results {
                    println!("{}", post.title);
                    println!("-----------\n");
                    println!("{}", post.body);
                }
            },
            Err(e) => println!("Error loading posts {}", e.to_string()),
        };
    }
    Ok(())
}