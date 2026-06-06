// Java の MemberSearchCondition.java 相当
// Option<T> が Java の @Nullable に対応する — null の代わりに None を使う
pub struct MemberSearchCondition {
    pub name: Option<String>,
    pub company_position_id: Option<i32>,
}
