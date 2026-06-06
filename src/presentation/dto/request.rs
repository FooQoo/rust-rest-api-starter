use serde::Deserialize;

use crate::domain::model::MemberSearchCondition;

// ─────────────────────────────────────────────
// 社員検索リクエスト
// ─────────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub(crate) struct MemberSearchRequest {
    // 必須。パラメータ未指定なら axum が 422 を自動返却
    pub(crate) name: String,

    // optional。Option<i32>: 未指定なら None、指定なら Some(値)
    pub(crate) company_position_id: Option<i32>,
}

impl MemberSearchRequest {
    // リクエスト DTO を消費してドメインの検索条件に変換
    pub(crate) fn into_condition(self) -> MemberSearchCondition {
        MemberSearchCondition {
            name: Some(self.name),
            company_position_id: self.company_position_id,
        }
    }
}
