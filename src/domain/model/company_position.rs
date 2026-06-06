// Java の CompanyPosition.java 相当
// Lombok の @Getter, @Builder はRustでは不要 — フィールドが pub なら直接アクセスできる
pub struct CompanyPosition {
    pub company_position_id: i32,
    pub name: String,
}
