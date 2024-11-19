use aide::operation::OperationIo;
use axum::response::IntoResponse;
use axum_macros::FromRequest;
use schemars::JsonSchema;
use serde::Serialize;

use crate::framework::api_doc::errors::AppError;

#[derive(FromRequest, OperationIo, JsonSchema)]
#[from_request(
    via(crate::framework::api_doc::axum_json_for_schema::Json),
    rejection(AppError)
)]
#[aide(
    input_with = "crate::framework::api_doc::axum_json_for_schema::Json<T>",
    output_with = "crate::framework::api_doc::axum_json_for_schema::Json<T>",
    json_schema
)]
pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
