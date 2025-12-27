//! Anthropic API Handler 函数

use axum::{http::StatusCode, response::IntoResponse, Json};

use super::types::{CountTokensRequest, ErrorResponse, MessagesRequest};

/// GET /v1/models
///
/// 返回可用的模型列表
/// 当前返回 501 Not Implemented
pub async fn get_models() -> impl IntoResponse {
    tracing::info!("Received GET /v1/models request");

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse::not_implemented("GET /v1/models not implemented")),
    )
}

/// POST /v1/messages
///
/// 创建消息（对话）
/// 当前返回 501 Not Implemented
pub async fn post_messages(Json(payload): Json<MessagesRequest>) -> impl IntoResponse {
    tracing::info!(
        model = %payload.model,
        max_tokens = %payload.max_tokens,
        stream = %payload.stream,
        message_count = %payload.messages.len(),
        "Received POST /v1/messages request"
    );

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse::not_implemented("POST /v1/messages not implemented")),
    )
}

/// POST /v1/messages/count_tokens
///
/// 计算消息的 token 数量
/// 当前返回 501 Not Implemented
pub async fn count_tokens(Json(payload): Json<CountTokensRequest>) -> impl IntoResponse {
    tracing::info!(
        model = %payload.model,
        message_count = %payload.messages.len(),
        "Received POST /v1/messages/count_tokens request"
    );

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse::not_implemented(
            "POST /v1/messages/count_tokens not implemented",
        )),
    )
}
