use diesel::prelude::*;
use crate::schema::posts;

// NOTE: Queryable assumes order of fields in Struct matches column order
// Using #[derive(Selectable)] in combination with SelectableHelper::as_select
// ensures the field order always matches

// Queryable generates code to load Post struct from SQL Query
// Selectable generate code to construct a SELECT clause from table_name 
#[derive(Queryable, Selectable)] 
#[diesel(table_name = crate::schema::posts)] 
#[diesel(check_for_backend(diesel::pg::Pg))] // adds compile-time checks
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}