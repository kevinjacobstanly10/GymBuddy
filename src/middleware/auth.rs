use axum::{
    async_trait,
    extract::{FromRequestParts},
    http::{request::Parts, StatusCode},
};
use crate::jwt::verify_jwt;

pub struct AuthUser {
    pub user_id: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts, 
        _state: &S
    ) -> Result<Self, Self::Rejection> {

        // Expect: Authorization: Bearer <token>
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

        // Extract token
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid auth header"));
        }

        let token = &auth_header[7..];

        // Verify token
        match verify_jwt(token) {
            Some(user_id) => Ok(AuthUser { user_id }),
            None => Err((StatusCode::UNAUTHORIZED, "Invalid or expired token")),
        }
    }
}
