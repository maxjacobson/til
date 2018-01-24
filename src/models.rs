extern crate chrono;

use super::schema::tils;

use self::chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct TIL {
    pub id: i32,
    pub contents: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "tils"]
pub struct NewTIL<'a> {
    pub contents: &'a str,
}
