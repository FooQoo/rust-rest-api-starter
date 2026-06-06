use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::model::{Count, Member};

// ─────────────────────────────────────────────
// 1 件の社員レスポンス
// ─────────────────────────────────────────────
// ToSchema: JSON ボディの OpenAPI schema を自動生成。
#[derive(Debug, Serialize, ToSchema)]
pub struct MemberResponse {
    /// 社員 ID
    pub id: i32,
    /// 社員名
    pub name: String,
    /// 役職名
    pub position_name: String,
}

impl From<Member> for MemberResponse {
    fn from(member: Member) -> Self {
        Self {
            id: member.id,
            name: member.name,
            position_name: member.company_position.name,
        }
    }
}

// ─────────────────────────────────────────────
// 社員リストレスポンス
// ─────────────────────────────────────────────
#[derive(Debug, Serialize, ToSchema)]
pub struct MemberListResponse {
    /// 社員リスト
    pub members: Vec<MemberResponse>,
}

impl From<Vec<Member>> for MemberListResponse {
    fn from(members: Vec<Member>) -> Self {
        Self {
            members: members.into_iter().map(MemberResponse::from).collect(),
        }
    }
}

// ─────────────────────────────────────────────
// 社員数レスポンス
// ─────────────────────────────────────────────
#[derive(Debug, Serialize, ToSchema)]
pub struct MemberCountResponse {
    /// 社員数
    pub count: u32,
}

impl From<Count> for MemberCountResponse {
    fn from(count: Count) -> Self {
        Self {
            count: count.value(),
        }
    }
}
