use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use http::Method;
use crate::spring::RBAC::authentication::{Authentication, AuthenticationManager, AuthenticationRequest};
use crate::spring::RBAC::security_context::SercurityContext;

pub async fn authentication_middleware (
    State(auth_manager): State<Box<dyn AuthenticationManager>>,
    mut req: Request,,
    next: Next
) -> Result<Response, StatusCode> {
    if req.method() == Method::GET && req.uri().path() == "/login" {
        let body = req.body_mut();
        let auth_req = AuthenticationRequest {
            username: "admin".to_string(),
            password: "123456".to_string(),
        };

        match auth_manager.authentication(&auth_req) {
            Ok(auth) => {
                SercurityContext::set_authentication(auth);
                Ok(next.run(req).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Ok(next.run(req).await)
    }
}