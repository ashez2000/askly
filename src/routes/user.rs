use rand::Rng;
use uuid::Uuid;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    domain::user::{NewUser, User},
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
