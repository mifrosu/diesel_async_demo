use diesel_async_demo::*;
use std::io::{stdin, Read};

// cargo run --bin write_post
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = db_url();
    let connection = &mut establish_connection(&url).await?;

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // remove trailing newline

    println!(
        "\nOK! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    match create_post(connection, title, &body).await {
        Ok(post) => println!("\nSaved draft {} with id {}", title, post.id),
        Err(e) => println!("\nWe were not able to save the Post!\n {}", e.to_string()),
    };
    Ok(())
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";