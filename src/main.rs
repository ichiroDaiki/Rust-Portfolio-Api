//rust analyzer indica error pero no hay un error como tal
use actix_web::{http::header, middleware::Logger, HttpResponse,web, App, HttpServer, Responder};
use actix_cors::Cors;
use diesel::prelude::*;
pub mod models;
pub mod schema;
use serde_json::json;
use crate::models::Projects;

#[macro_use] 
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
// use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::{env}; 

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn build_response(
    _status_code: String,
    _success: bool,
    data: Vec<models::Projects>,
) -> serde_json::Value {
    let parsed_result = json!({
        "statusCode": _status_code,
        "success": _success,
        "data": data
    });

    return parsed_result;
}

async fn index() -> impl Responder {
    use self::schema::projects::dsl::*;

    let connection = establish_connection();
    let results = projects
        .load::<Projects>(&connection)
        .expect("Error on method GET on Index");

    let len_result = results.len();
    let mut _response = json!({});

    if len_result >= 1 {
        _response = build_response(String::from("200"), true, results);
    }

    if len_result == 0 {
        _response = build_response(String::from("204"), true, vec![]);
    }

   HttpResponse::Ok().json(&_response);
   format!("{}", _response)

    // HttpResponse::Ok().json(&_response)
}

#[actix_web::main]

async fn main() -> std::io::Result<()>{
    println!("Iniciando Servidor");

        // Get the port number to listen on.
        let port = env::var("PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse()
        .expect("PORT must be a number"); 

    HttpServer::new(move || {
        App::new()//.app_data(pool.clone())
        .wrap(
            Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600),
        )
        .wrap(Logger::default()) 
        .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", port))
    .expect("error puertos")
    .run().await
}
