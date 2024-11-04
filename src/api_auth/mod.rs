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
    }; // ($error:path,$message:literal) => {
       //     #[allow(unused)]
       //     impl From<$error> for AuthError {
       //         fn from(value: $error) -> Self {
       //             #[cfg(feature = "dev")]
       //             {
       //                 let backtrace = std::backtrace::Backtrace::capture();
       //                 tracing::error!(
       //                     "error occurred: position: {:?} ; error:{value}",
       //                     backtrace.frames()[4]
       //                 );
       //             }
       //             AuthError(AppError {
       //                 error: format!("error:::::::: {}", $message),
       //                 error_id: Default::default(),
       //                 status: Default::default(),
       //                 error_details: None,
       //             })
       //         }
       //     }
       // };
}
