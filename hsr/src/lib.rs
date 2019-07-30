pub use actix_web;
pub use actix_http;
pub use actix_rt;
pub use awc;
pub use futures1;
pub use futures3;
pub use serde;
pub use serde_urlencoded;
pub use url;

// We re-export this type as it is used in all the trait functions
pub use futures3_core::future::{LocalBoxFuture as LocalBoxFuture3};

use actix_web::{Either, HttpRequest, HttpResponse, Responder, Error as AxError};
use actix_http::http::StatusCode;
use std::fmt;

#[doc(hidden)]
pub enum Void {}

impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for Void {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl Responder for Void {
    type Error = ();
    type Future = Result<HttpResponse, ()>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        unreachable!()
    }
}

impl actix_web::ResponseError for Void {}

/// Associate an http status code with a type. Defaults to 501 Internal Server Error
pub trait HasStatusCode {
    /// The http status code associated with the type
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub fn result_to_either<A, B>(res: Result<A, B>) -> Either<A, B> {
    match res {
        Ok(a) => Either::A(a),
        Err(b) => Either::B(b),
    }
}

pub trait Error: HasStatusCode {}

#[derive(Debug)]
pub enum ClientError {
    BadStatus(StatusCode),
    Actix(AxError)
}

impl HasStatusCode for ClientError {}
impl Error for ClientError {}