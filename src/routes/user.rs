use std::future;

use chrono::{prelude::*, Days};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply, Filter};

use crate::{
    domain::user::{AuthPayload, Credential, NewUser, User},
    error::Error,
    store::DbStore,
};

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = argon2::Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn signup(store: DbStore, input: NewUser) -> Result<impl Reply, Rejection> {
    let hashed_password = hash_password(input.password.as_bytes());

    let user = User {
        id: Uuid::new_v4(),
        name: input.name,
        email: input.email,
        password: hashed_password,
    };

    match store.add_user(user).await {
        Ok(_) => Ok(warp::reply::json(&true)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn signin(store: DbStore, input: Credential) -> Result<impl Reply, Rejection> {
    match store.find_user_by_credential(input).await {
        Ok(user) => {
            let token = sign_token(user.id)?;
            Ok(warp::reply::json(&token))
        }

        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn sign_token(user_id: Uuid) -> Result<String, Error> {
    let now = Utc::now();
    let exp = now.checked_add_days(Days::new(7)).unwrap();

    let payload = AuthPayload {
        user_id,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|_| Error::JwtError)
}

fn verify_token(token: String) -> Result<AuthPayload, Error> {
    let payload = decode::<AuthPayload>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::JwtError)?;

    Ok(payload.claims)
}

pub fn protect() -> impl Filter<Extract = (AuthPayload,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let payload = match verify_token(token) {
            Ok(t) => t,
            Err(e) => return future::ready(Err(warp::reject::custom(e))),
        };

        future::ready(Ok(payload))
    })
}
