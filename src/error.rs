use std::convert::Infallible;

use warp::{
    http::StatusCode,
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug)]
pub enum Error {
    NotFound(String),
}

impl Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(Error::NotFound(s)) = err.find() {
        Ok(warp::reply::with_status(
            s.to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
