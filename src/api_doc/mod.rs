use aide::openapi::Tag;
use aide::transform::{TransformOpenApi, TransformOperation};
use axum::http::Uri;
use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;

use crate::api_doc::errors::AppError;
use crate::api_doc::extractors::Json;

pub mod axum_json_for_schema;
pub mod docs;
pub mod errors;
pub mod extractors;

pub fn default_resp_docs<Resp: JsonSchema + Serialize>(
    op: TransformOperation,
) -> TransformOperation {
    op.description("default_docs").response::<200, Json<Resp>>()
}

// pub fn default_resp_docs_with_exam<Resp: JsonSchema + Serialize>(
//     op: TransformOperation,
// ) -> TransformOperation {
//     op.description("default_docs").response::<200, Json<Resp>>()
// }

pub fn empty_resp_docs(op: TransformOperation) -> TransformOperation {
    op.description("default_docs")
    // .response::<200,Json<Resp>>()
}
pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("RsWebQuicStar")
        .summary("RsWebQuicStar")
        .description(include_str!("../api-doc.md"))
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<axum::Json<AppError>, _>(|res| {
            res.example(
                AppError::new("some error happened".to_string())
                    .with_status(StatusCode::IM_A_TEAPOT),
            )
        })
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
