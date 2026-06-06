// Java の MemberEntity.java 相当
// SQLx が DB の行を直接マッピングするための構造体
//
// FromRow デリベー: sqlx::query_as::<_, MemberRow>() 実行時に
// 自動的にカラム名 → フィールド名でマッピングしてくれる
#[derive(Debug, sqlx::FromRow)]
pub(crate) struct MemberRow {
    pub(crate) member_id: i32,
    pub(crate) name: String,
    pub(crate) company_position_id: i32,
    pub(crate) position_name: String,
}
