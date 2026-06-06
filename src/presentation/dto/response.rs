use serde::Serialize;

use crate::domain::model::{Count, Member};

// ─────────────────────────────────────────────
// 1 件の社員レスポンス
// ─────────────────────────────────────────────
#[derive(Debug, Serialize)]
pub(crate) struct MemberResponse {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) position_name: String,
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
#[derive(Debug, Serialize)]
pub(crate) struct MemberListResponse {
    pub(crate) members: Vec<MemberResponse>,
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
#[derive(Debug, Serialize)]
pub(crate) struct MemberCountResponse {
    pub(crate) count: u32,
}

impl From<Count> for MemberCountResponse {
    fn from(count: Count) -> Self {
        Self {
            count: count.value(),
        }
    }
}
