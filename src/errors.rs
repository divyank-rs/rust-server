use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Servor Error")]
    InternalServorError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "No Content")]
    NoContent,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            ServiceError::InternalServorError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(x) => HttpResponse::BadRequest().json(x),
            ServiceError::NoContent => HttpResponse::NoContent().json("No content"),
        }
    }
}
