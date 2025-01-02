use std::sync::Arc;
use actix_web::{web, Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use futures::future::{ready, Ready};
use getset::Getters;
use jsonwebtoken::{decode, decode_header, Algorithm, Validation};
use serde::{Deserialize, Serialize};
use crate::inbound::http::AuthState;

#[derive(Debug, Serialize, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct Claims {
    exp: usize,
    preferred_username: String,
    realm_access: RealmAccess,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmAccess {
    roles: Vec<String>,
}

#[derive(Debug)]
pub struct KeycloakToken(Claims);

impl KeycloakToken {
    pub fn claims(&self) -> &Claims {
        &self.0
    }
}

impl FromRequest for KeycloakToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_state = req.app_data::<web::Data<AuthState>>().unwrap();
        let validation = Arc::clone(&auth_state.validator);
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        if let Some(token) = auth_header {

            match decode_header(token) {
                Ok(header) => {
                    if let Some(kid) = header.kid {

                        if let Some(decoding_key) = auth_state.auth_keys.get(&kid) {
                            let result  = decode::<Claims>(token, decoding_key, &validation);
                            return match result {
                                Ok(token_data) => ready(Ok(KeycloakToken(token_data.claims))),
                                Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
                            };
                        }
                    }
                }
                Err(err) => {
                    return ready(Err(actix_web::error::ErrorUnauthorized("Invalid JWT header")))
                },
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header")))
    }
}