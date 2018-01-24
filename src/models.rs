use super::schema::tils;

#[derive(Insertable)]
#[table_name = "tils"]
pub struct NewTIL<'a> {
    pub contents: &'a str,
}
