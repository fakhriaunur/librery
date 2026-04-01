use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use biscuit_auth::{Biscuit, builder::AuthorizerBuilder};

use crate::AppState;

#[derive(Clone, Debug)]
pub struct AuthenticatedPatron {
    pub patron_id: String,
}

fn extract_auth_token(req: &Request) -> Option<String> {
    req.headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with("auth_token=") {
                        Some(cookie["auth_token=".len()..].to_string())
                    } else {
                        None
                    }
                })
        })
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_auth_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;

    let state = req
        .extensions()
        .get::<AppState>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let biscuit = Biscuit::from_base64(&token, state.biscuit_keypair.public())
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut authorizer = AuthorizerBuilder::new()
        .time()
        .build(&biscuit)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    authorizer
        .authorize()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let patron_id: Vec<(String,)> = authorizer
        .query("data($id) <- patron($id)")
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let patron_id = patron_id
        .into_iter()
        .next()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .0;

    req.extensions_mut().insert(AuthenticatedPatron {
        patron_id: patron_id.to_string(),
    });

    Ok(next.run(req).await)
}
