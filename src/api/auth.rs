use crate::framework::api_doc::extractors::Json;
use crate::framework::auth::{AuthBackend, Credentials};
use aide::axum::IntoApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum_login::AuthSession;

pub async fn login(
    mut auth_session: AuthSession<AuthBackend>,
    Json(creds): Json<Credentials>,
) -> impl IntoApiResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Some(ref next) = creds.next {
        Redirect::to(next).into_response()
    } else {
        StatusCode::OK.into_response()
    }
}
