use std::env;
use std::io::ErrorKind;
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::Algorithm::HS256;
use rocket::http::Status;
use rocket::outcome::Outcome::Error;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

use crate::model;
use crate::model::usermodel::Token;

pub struct JwtAuth;

impl<'r> FromRequest<'r> for JwtAuth{

    type Error = std::io::Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,Self::Error> {
        let token = request.headers().get_one("acces_Token");
        match token {
            None => {
                Outcome::Error((Status::Unauthorized, std::io::Error::new(ErrorKind::PermissionDenied, ())))
            }
            Some(token) => {
                let key = "JWT_SECRET";
                let val = env::var(key).unwrap();
                let secret = DecodingKey::from_secret(val.as_ref());
                let validation = Validation::new(HS256);
                match decode::<Token>{token&, &Validation::new(Algorithm::HS256)  }
            }
        }
    }
}