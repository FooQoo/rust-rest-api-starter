use serde::Serialize;

use crate::domain::model::member::Member;

// Java の MemberResponse record 相当
// Serialize: 構造体 → JSON に変換 (Jackson の @JsonProperty 相当)
#[derive(Serialize)]
pub struct MemberResponse {
    pub id: i32,
    pub name: String,
    pub position_name: String,
}

impl From<Member> for MemberResponse {
    // Java の MemberResponse.from(Member) 静的ファクトリメソッド相当
    // From トレイトを実装すると Into も自動で使えるようになる
    fn from(member: Member) -> Self {
        Self {
            id: member.id,
            name: member.name,
            position_name: member.company_position.name,
        }
    }
}
