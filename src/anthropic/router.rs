//! Anthropic API 路由配置

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::{
    handlers::{count_tokens, get_models, post_messages},
    middleware::{auth_middleware, cors_layer, AppState},
};

/// 创建 Anthropic API 路由
///
/// # 端点
/// - `GET /v1/models` - 获取可用模型列表
/// - `POST /v1/messages` - 创建消息（对话）
/// - `POST /v1/messages/count_tokens` - 计算 token 数量
///
/// # 认证
/// 所有 `/v1` 路径需要 API Key 认证，支持：
/// - `x-api-key` header
/// - `Authorization: Bearer <token>` header
pub fn create_router(api_key: impl Into<String>) -> Router {
    let state = AppState::new(api_key);

    // 需要认证的 /v1 路由
    let v1_routes = Router::new()
        .route("/models", get(get_models))
        .route("/messages", post(post_messages))
        .route("/messages/count_tokens", post(count_tokens))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/v1", v1_routes)
        .layer(cors_layer())
        .with_state(state)
}
