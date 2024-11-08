use crate::api_auth::login_impl::AuthBackend;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub mod login_impl;
pub mod router;

#[test]
pub fn test() {
    println!("{}", password_auth::generate_hash("1234qwer"));
}
#[macro_export]
macro_rules! impl_from {
    ($error:path) => {
        impl From<$error> for AuthError {
            fn from(value: $error) -> Self {
                AuthError(AppError::new(format!("error:::::::: {}", value)))
            }
        }
    };
}

pub fn get_auth_layer(
    connection_pool: Pool<ConnectionManager<PgConnection>>,
) -> AuthManagerLayer<AuthBackend, MemoryStore> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = AuthBackend::new(connection_pool);
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
