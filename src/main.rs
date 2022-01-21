use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use self::models::*;
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
struct Data {
    nombre: String,
}

#[derive(Deserialize, Serialize)]
struct ServerStatus{
    status: String,
    message: String,
    data : Vec<std::string::String>,
}


#[post("/registro")]
async fn registro(req_body: web::Json<Data>) -> impl Responder {


    let new_user = NewUser{
        nombre: &req_body.nombre,
    };
  
    if new_user.nombre != "" {

        let data_json  = vec![serde_json::to_string(&new_user).unwrap()];
        
        let new_status = ServerStatus{
            status : String::from("200"),
            message :  String::from("Datos Recibidos"),
            data : data_json
        };

        let crear_usuario = create_user(&new_user);
        println!("SQL Result -> {:?}", serde_json::to_string(&crear_usuario).unwrap());
        
        let serialized_data = serde_json::to_string(&new_status).unwrap();
        HttpResponse::Ok().body(serialized_data)

    }else{

        let new_status = ServerStatus{
            status : String::from("200"),
            message :  String::from("Datos Recibidos"),
            data : vec![]
        };

        let serialized_data = serde_json::to_string(&new_status).unwrap();

        HttpResponse::Ok().body(serialized_data)
    }
    
 
}

fn create_user(new_user : &NewUser) -> Usuarios{

    use self::schema::usuarios::dsl::*;
    let connection = establish_connection();

                   
    diesel::insert_into(usuarios)
    .values(new_user)
    .get_result(&connection)
    .expect("Error saving new post")
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


async fn index() -> impl Responder {
    use self::schema::usuarios::dsl::*;

    let connection = establish_connection();
    let results = usuarios
        .limit(5)
        .load::<Usuarios>(&connection)
        .expect("Error loading posts");


    println!("Displaying {} posts", results.len());
    for usuario in &results {
        println!("Nombre -> {}", usuario.nombre);
    }

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(registro)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}