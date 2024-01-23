pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};

use self::models::{NewPost, Post};

use futures_util::future::BoxFuture;
use futures_util::FutureExt;

pub fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        AsyncPgConnection::establish(config).await
    };
    fut.boxed()
}

pub fn db_url() -> String {
    match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => "postgres://postgres:postgres@localhost:5432/diesel_async_demo".to_string(),
    }
}

pub async fn create_post(conn: &mut AsyncPgConnection, title: &str, body: &str) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .await
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
