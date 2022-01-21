use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Post{

    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,

}


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Usuarios{
    pub nombre: String,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Deserialize, Serialize)]
#[derive(Insertable)]
#[table_name="usuarios"]
pub struct NewUser<'a> {
    pub nombre: &'a str,
}