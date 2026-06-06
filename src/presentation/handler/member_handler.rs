use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    error::AppError,
    presentation::dto::{
        request::MemberSearchRequest,
        response::{MemberCountResponse, MemberListResponse},
    },
    AppState,
};

// Java の MemberController#count() 相当
// State(state): axum の依存性注入。Spring の @Autowired に相当。
// Result<Json<T>, AppError>: 成功時は JSON、失敗時は AppError (HTTP 500)
#[tracing::instrument(skip(state), err)]
pub(crate) async fn count(
    State(state): State<AppState>,
) -> Result<Json<MemberCountResponse>, AppError> {
    let count = state.member_service.count().await?;
    Ok(Json(MemberCountResponse::from(count)))
}

// Java の MemberController#search() 相当
// Query(request): クエリパラメータを MemberSearchRequest にデシリアライズ
//   ?name=John&company_position_id=1 → MemberSearchRequest { name: "John", ... }
// name が欠けていると axum が自動で 422 Unprocessable Entity を返す
#[tracing::instrument(skip(state), err)]
pub(crate) async fn search(
    State(state): State<AppState>,
    Query(request): Query<MemberSearchRequest>,
) -> Result<Json<MemberListResponse>, AppError> {
    let members = state
        .member_service
        .search(request.into_condition())
        .await?;
    Ok(Json(MemberListResponse::from(members)))
}
