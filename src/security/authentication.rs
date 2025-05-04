use std::env;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use chrono::Utc;

use crate::model::usermodel::{Claims, NetworkResponse, Response, ResponseBody, JWT};

pub struct JwtAuth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("authorization");

        match token {
            None => {
                let response = Response {
                    body: ResponseBody::Message("Error validating JWT token - No token provided".to_string()),
                };
                Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()),
                ))
            }
            Some(key) => match decode_jwt(key.to_string()) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(kind) => {
                    let message = match kind {
                        jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Error validating JWT token - Expired Token",
                        jsonwebtoken::errors::ErrorKind::InvalidToken => "Error validating JWT token - Invalid Token",
                        _ => "Error validating JWT token - Unknown Error",
                    };

                    let response = Response {
                        body: ResponseBody::Message(message.to_string()),
                    };

                    Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()),
                    ))
                }
            },
        }
    }
}

pub fn create_jwt(id: i32) -> Result<String, Box<dyn std::error::Error>> {
    let secret = env::var("JWT_SECRET")?;
    let expiration_date = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .unwrap()
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration_date as usize,
    };

    let header = Header::new(Algorithm::HS512);

    Ok(encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn decode_jwt(token: String) -> Result<Claims, jsonwebtoken::errors::ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(data) => Ok(data.claims),
        Err(err) => Err(err.kind().clone()),
    }
}
