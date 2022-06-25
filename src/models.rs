use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Projects {
    pub id: i32,
    pub title: String,
    pub body: String, 
    pub gallery_name: String,
    pub name_tech: std::vec::Vec<String>,
}

/* #[derive(Serialize, Deserialize, Queryable)]
pub struct Usuarios {
    pub nombre: String,
}
/*  */
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
 */
/* #[derive(Deserialize, Serialize, Insertable)]
#[table_name = "usuarios"]
pub struct NewUser<'a> {
    pub nombre: &'a str,
}
 */
