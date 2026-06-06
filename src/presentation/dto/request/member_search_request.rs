use serde::Deserialize;

use crate::domain::model::member_search_condition::MemberSearchCondition;

// Java の MemberSearchRequest record 相当
// Deserialize: クエリパラメータ (?name=John&company_position_id=1) → 構造体に変換
#[derive(Deserialize)]
pub struct MemberSearchRequest {
    // Java の @NotBlank String name に相当
    // Option にしないことで、パラメータ未指定時は axum が 422 を返す
    pub name: String,

    // Java の Integer companyPositionId (nullable) に相当
    // Option<i32>: パラメータ未指定なら None、指定なら Some(値)
    pub company_position_id: Option<i32>,
}

impl MemberSearchRequest {
    // Java の toCondition() に相当
    pub fn into_condition(self) -> MemberSearchCondition {
        MemberSearchCondition {
            name: Some(self.name),
            company_position_id: self.company_position_id,
        }
    }
}
