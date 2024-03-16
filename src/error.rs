use std::convert::Infallible;

use warp::{
    http::StatusCode,
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug)]
pub enum Error {
    DbError(sqlx::Error),
    InvalidEmailPassword,
    ServerError,
}

impl Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(Error::InvalidEmailPassword) = err.find() {
        return Ok(warp::reply::with_status(
            "Invalid email or password".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    if let Some(Error::DbError(e)) = err.find() {
        // TODO: handle Db errors
        eprintln!("{}", e);

        return Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    if let Some(Error::ServerError) = err.find() {
        return Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    Ok(warp::reply::with_status(
        "Route not found".to_string(),
        StatusCode::NOT_FOUND,
    ))
}
