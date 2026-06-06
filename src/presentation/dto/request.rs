use serde::Deserialize;
use utoipa::IntoParams;

use crate::domain::model::MemberSearchCondition;

// ─────────────────────────────────────────────
// 社員検索リクエスト
// ─────────────────────────────────────────────
// IntoParams: クエリ/パスパラメータの OpenAPI schema を自動生成。
// (JSON ボディの場合は ToSchema を使う)
#[derive(Debug, Deserialize, IntoParams)]
pub struct MemberSearchRequest {
    /// 社員名 (完全一致)
    pub name: String,

    /// 部署 ID で絞り込み (任意)
    pub company_position_id: Option<i32>,
}

impl MemberSearchRequest {
    // リクエスト DTO を消費してドメインの検索条件に変換。
    // #[must_use]: 変換結果を捨てると警告 (使い忘れ防止)
    #[must_use]
    pub fn into_condition(self) -> MemberSearchCondition {
        MemberSearchCondition {
            name: Some(self.name),
            company_position_id: self.company_position_id,
        }
    }
}
