use std::fmt::Display;

use actix_web::{body::BoxBody, http::StatusCode, web, HttpResponse, Responder, ResponseError};

#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
    repsonse_code: StatusCode,
}

impl ApiResponse {
    pub fn new(status_code: u16, body: String) -> ApiResponse {
        ApiResponse {
            status_code,
            body,
            repsonse_code: StatusCode::from_u16(status_code).unwrap(),
        }
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.repsonse_code).set_body(body)
    }
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {} \n Status Code: {}",
            self.body, self.status_code
        )
    }
}

impl ResponseError for ApiResponse {
    fn status_code(&self) -> StatusCode {
        self.repsonse_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = BoxBody::new(web::BytesMut::from(self.body.as_bytes()));
        HttpResponse::new(self.status_code()).set_body(body)
    }
}
