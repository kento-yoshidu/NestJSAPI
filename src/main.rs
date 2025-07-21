mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use argon2::Argon2;
use password_hash::SaltString;
use rand::rngs::OsRng;
use sqlx::{postgres::PgPoolOptions, PgPool};
use password_hash::PasswordHasher;
use dotenv::dotenv;
use std::env;

use crate::models::user::{User, NewUser};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

fn hash_password(plain: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(plain.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

#[post("/users")]
async fn register_user(
    pool: web::Data<PgPool>,
    form: web::Json<NewUser>,
) -> impl Responder {
    let hash = match hash_password(&form.password) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().body("Password hash failed"),
    };

    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, hashed_password)
        VALUES ($1, $2)
        RETURNING id, name, hashed_password, created_at
        "#,
        form.name,
        hash
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("DB Error {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: http://localhost:8080");

    dotenv::dotenv().ok();

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(register_user)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
