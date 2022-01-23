use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use self::models::*;
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};
pub mod models;
pub mod schema;
use std::thread;


#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};


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

        create_user(&new_user);
    
        HttpResponse::Ok().json(&new_status)

    }else{

        let new_status = ServerStatus{
            status : String::from("200"),
            message :  String::from("No se registro ningun dato"),
            data : vec![]
        };


        HttpResponse::Ok().json(&new_status)
    }
    
 
}

#[post("/borrar-usuarios/{id}")]
async fn delete_user(web::Path(id_temp): web::Path<String>) -> impl Responder{
  
        if delete_user_service(id_temp) {

                let message : String = "Se elimino el registro".to_owned();
                let new_status = ServerStatus{
                    status : String::from("200"),
                    message :  String::from(message),
                    data : vec![]
                };

                HttpResponse::Ok().json(&new_status)                

        }else{

               let message : String = "No se encontro el registro".to_owned();
    
                let new_status = ServerStatus{
                    status : String::from("200"),
                    message :  String::from(message),
                    data : vec![]
                };

                HttpResponse::Ok().json(&new_status) 

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

fn delete_user_service(id_temp : String) -> bool{

    let connection = establish_connection();

    let mut sql_select : String = "SELECT id FROM usuarios WHERE id = ".to_owned();
    let parameter_select : String = id_temp.to_owned();

    sql_select.push_str(&parameter_select);

    let _query_select = diesel::sql_query(sql_select).execute(&connection);
    let mut response = false;

    for identificadores in &_query_select {

        if identificadores.to_string() == "1"{
            
            let mut sql : String = "DELETE FROM usuarios WHERE id = ".to_owned();
            let parameter : String = id_temp.to_owned();
        
            sql.push_str(&parameter);
        
            let _query = diesel::sql_query(sql).execute(&connection);

            response = true;

        }else{
            
            response = false;

        }
    }

    response

}


fn establish_connection() -> PgConnection {
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
        .load::<Usuarios>(&connection)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Iniciando Servidor -> 127.0.0.1:8080");
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres password=tribalmaxg516".parse().unwrap(),
        NoTls,
    );
    
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new().data(pool.clone())
            .service(registro)
            .service(delete_user)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}