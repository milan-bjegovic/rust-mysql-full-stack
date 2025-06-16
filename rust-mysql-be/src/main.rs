use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use sqlx::MySqlPool;
use std::env;
use dotenv::dotenv;

#[derive(sqlx::FromRow, serde::Serialize)]
struct User {
    id: i32,
    name: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&db_url)
        .await
        .expect("Failed to connect to MySQL");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(hello))
            .route("/users", web::get().to(get_users))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust + MySQL!")
}

async fn get_users(pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let users = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("DB error: {}", e);
            HttpResponse::InternalServerError().body("Error fetching users")
        }
    }
}
