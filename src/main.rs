mod models;
mod handlers;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use bcrypt::verify;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;

use crate::models::user::{Claims, LoginRequest, NewUser, User};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("signup")]
async fn signup(db_pool: web::Data<sqlx::PgPool>, paylod: web::Json<NewUser>) -> impl Responder {
    let hashed = bcrypt::hash(&paylod.password, bcrypt::DEFAULT_COST).unwrap();

    let result = sqlx::query!(
        "INSERT INTO users
            (name, hashed_password)
        VALUES
            ($1, $2)",
        paylod.name,
        hashed
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("ユーザー登録完了"),
        Err(e) => {
            eprintln!("DB保存エラー: {:?}", e);
            HttpResponse::InternalServerError().body("保存失敗")
        }
    }
}

#[post("signin")]
pub async fn signin(db_pool: web::Data<sqlx::PgPool>, form: web::Json<LoginRequest>) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        "SELECT
            id,
            name,
            hashed_password,
            created_at
        FROM
            users
        WHERE
            name = $1"
    )
    .bind(&form.name)
    .fetch_optional(db_pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        _ => return HttpResponse::Unauthorized().body("ユーザーが見つかりません"),
    };

    let is_valid = verify(&form.password, &user.hashed_password).unwrap_or(false);
    if !is_valid {
        return HttpResponse::Unauthorized().body("パスワードが間違っています");
    }

    // JWT生成
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp();

    let claims = Claims {
        user_id: user.id,
        exp: expiration as usize,
    };

    const SECRET: &[u8] = b"secret";

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
        .unwrap();

    HttpResponse::Ok().json(serde_json::json!({ "token": token }))
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
            .service(signin)
            .service(signup)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
