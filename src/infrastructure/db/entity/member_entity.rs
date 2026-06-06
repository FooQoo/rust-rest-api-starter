// Java の MemberEntity.java 相当
// SQLx が DB の行を直接マッピングするための構造体
//
// FromRow デリベー: sqlx::query_as::<_, MemberRow>() 実行時に
// 自動的にカラム名 → フィールド名でマッピングしてくれる
#[derive(sqlx::FromRow)]
pub struct MemberRow {
    pub member_id: i32,
    pub name: String,
    pub company_position_id: i32,
    pub position_name: String,
}
