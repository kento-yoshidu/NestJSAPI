use actix_web::{dev::ServiceRequest, HttpRequest, HttpResponse, HttpMessage, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use crate::models::user::Claims;

const SECRET: &[u8] = b"secret";

pub async fn validate_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token_data = decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    );

    match token_data {
        Ok(data) => {
            req.extensions_mut().insert(data.claims);
            Ok(req)
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let key = DecodingKey::from_secret(SECRET);
    let validation = Validation::new(Algorithm::HS256);
    let token_data: TokenData<Claims> = decode(token, &key, &validation)?;

    Ok(token_data.claims)
}

pub fn extract_user_from_jwt(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header_str| header_str.strip_prefix("Bearer "))
        .map(String::from);

    let token = match token {
        Some(t) => t,
        None => return Err(HttpResponse::Unauthorized().body("Missing Authorization token")),
    };

    match decode_jwt(&token) {
        Ok(claims) => Ok(claims),
        Err(_) => Err(HttpResponse::Unauthorized().body("Invalid token")),
    }
}
