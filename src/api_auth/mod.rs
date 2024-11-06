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
