use crate::framework::api_doc::errors::AppError;
use crate::framework::api_doc::extractors::Json;
use crate::AppRes;
use aide::OperationIo;
use axum::extract::State;
use axum_login::{login_required, AuthSession};
use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Insertable, Queryable, RunQueryDsl, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use axum_login::permission_required;

use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::db_models;
use crate::db_models::user::web::get_routers;
use crate::db_models::user::User;
use crate::db_models::ConnPool;
use crate::framework::api_doc::{default_resp_docs, empty_resp_docs};
use crate::framework::auth::AuthBackend;
use crate::schema::users::dsl::users;

#[derive(Serialize, Deserialize, OperationIo, Debug, Default, JsonSchema)]
struct ModifyPassword {
    old_password: String,
    new_password: String,
}
pub(crate) async fn modify_password(
    State(pool): State<ConnPool>,
    auth_session: AuthSession<AuthBackend>,
    Json(modify_password): Json<ModifyPassword>,
) -> AppRes<String> {
    if modify_password.new_password.len() < 8 {
        return Err(AppError::new(
            "password should be longer than or equal 8".to_string(),
        ));
    }
    match auth_session.user {
        None => return Err(AppError::new("not be".to_string())),
        Some(mut user) => {
            password_auth::verify_password(modify_password.old_password, &user.password)?;
            let hash = password_auth::generate_hash(modify_password.new_password);
            user.password = hash;
            diesel::update(users).set(user).execute(&mut pool.get()?)?;
        }
    }

    Ok("succeed".to_string())
}
