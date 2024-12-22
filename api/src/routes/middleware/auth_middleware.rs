use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    middleware::Next,
    Error, HttpMessage,
};

use crate::utils::{api_response, jwt::decode_jwt};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Parse auth headers
    let auth = req.headers().get(AUTHORIZATION);

    // If none are provided, return unauthorized
    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(
            401,
            "Unauthorized".to_string(),
        )));
    }

    // Parse token
    let token = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "")
        .to_owned();

    // Decode the token
    let claim = decode_jwt(token).unwrap();

    // Inserting the claim
    req.extensions_mut().insert(claim.claims);

    // Captures any errors with the request
    next.call(req)
        .await
        .map_err(|err| Error::from(api_response::ApiResponse::new(500, err.to_string())))
}
