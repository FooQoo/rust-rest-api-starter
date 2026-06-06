use crate::domain::model::{CompanyPosition, Member};

// Java の MemberEntity.java 相当
// SQLx が DB の行を直接マッピングするための構造体
//
// FromRow デリベー: sqlx::query_as::<_, MemberRow>() 実行時に
// 自動的にカラム名 → フィールド名でマッピングしてくれる
#[derive(Debug, sqlx::FromRow)]
pub struct MemberRow {
    pub member_id: i32,
    pub name: String,
    pub company_position_id: i32,
    pub position_name: String,
}

// DB 行 (MemberRow) → ドメインモデル (Member) への変換を集約。
//
// 依存方向のポイント:
//   - Member は domain 層 (純粋)
//   - MemberRow は infrastructure 層 (DB スキーマと結合)
//   - From<MemberRow> for Member の実装は infrastructure 側に置く
//     (ドメインがインフラを知らない形を保つ)
impl From<MemberRow> for Member {
    fn from(row: MemberRow) -> Self {
        Self {
            id: row.member_id,
            name: row.name,
            company_position: CompanyPosition {
                company_position_id: row.company_position_id,
                name: row.position_name,
            },
        }
    }
}
